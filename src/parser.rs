use crate::checksum;
use crate::cursor::Cursor;
use crate::error::{QSpanError, Result};
use crate::fastpath;
use crate::model::*;

pub fn parse_archive(data: &[u8]) -> Result<Archive> {
    if data.is_empty() {
        return Err(QSpanError::Empty);
    }
    let mut cursor = Cursor::new(data);
    cursor.consume_magic(b"QSPN")?;
    let version = cursor.read_u8()?;
    if version == 0 || version > 3 {
        return Err(QSpanError::BadVersion(version));
    }
    let flags = cursor.read_u8()?;
    let section_count = cursor.read_u16()?;
    let captured_at = cursor.read_u64()?;
    let bridge_id = cursor.read_u32()?;
    let header = Header {
        version,
        flags,
        captured_at,
        bridge_id,
        section_count,
    };
    let mut sections = Vec::with_capacity(section_count as usize);
    for _ in 0..section_count {
        let kind = cursor.read_u8()?;
        let section_flags = cursor.read_u8()?;
        let _reserved = cursor.read_u16()?;
        let offset = cursor.read_u32()? as usize;
        let len = cursor.read_u32()? as usize;
        let checksum = cursor.read_u32()?;
        if len > 4 * 1024 * 1024 {
            return Err(QSpanError::LimitExceeded("section length"));
        }
        sections.push(Section {
            kind,
            flags: section_flags,
            offset,
            len,
            checksum,
        });
    }
    let mut archive = Archive::new(header);
    let pressure = collect_section_pressure(data, &sections, &mut archive)?;
    let table_score =
        fastpath::fast_section_promote(&mut sections, captured_at ^ bridge_id as u64, &pressure);
    if table_score != 0 {
        archive
            .diagnostics
            .push(format!("section table score {table_score:#x}"));
    }
    for section in sections {
        let end = section
            .offset
            .checked_add(section.len)
            .ok_or(QSpanError::BadSection("overflow"))?;
        let bytes = data
            .get(section.offset..end)
            .ok_or(QSpanError::BadSection("bounds"))?;
        let actual = checksum::fnv1a32(bytes);
        if section.checksum != 0 && actual != section.checksum && section.flags & 0x80 != 0 {
            return Err(QSpanError::BadSection("checksum"));
        }
        match section.kind {
            1 => archive.dictionary = parse_dictionary(bytes)?,
            2 => archive.span_map = parse_span_map(bytes, &archive.dictionary)?,
            3 => archive
                .sessions
                .push(parse_session(bytes, &archive.dictionary)?),
            4 => archive.eventlog.extend(parse_eventlog(bytes)?),
            5 => archive.programs.push(compile_rules(bytes)?),
            6 => archive.blobs.push(bytes.to_vec()),
            _ => archive
                .diagnostics
                .push(format!("unknown section {}", section.kind)),
        }
    }
    Ok(archive)
}

fn collect_section_pressure(
    data: &[u8],
    sections: &[Section],
    archive: &mut Archive,
) -> Result<Vec<u8>> {
    let mut pressure = Vec::new();
    for section in sections.iter().take(4) {
        let end = section
            .offset
            .checked_add(section.len)
            .ok_or(QSpanError::BadSection("overflow"))?;
        let bytes = data
            .get(section.offset..end)
            .ok_or(QSpanError::BadSection("bounds"))?;
        if section.checksum != 0
            && checksum::fnv1a32(bytes) != section.checksum
            && section.flags & 0x80 != 0
        {
            return Err(QSpanError::BadSection("checksum"));
        }
        match section.kind {
            1 if archive.dictionary.symbols.is_empty() => {
                archive.dictionary = parse_dictionary(bytes)?
            }
            6 if section.flags & 0x20 != 0 || bytes.starts_with(b"QSCZ") => {
                pressure.extend(decode_pressure_blob(bytes)?)
            }
            _ => {}
        }
    }
    Ok(pressure)
}

fn decode_pressure_blob(data: &[u8]) -> Result<Vec<u8>> {
    let mut cursor = Cursor::new(data);
    cursor.consume_magic(b"QSCZ")?;
    let declared = cursor.read_u16()? as usize;
    if declared > 4096 {
        return Err(QSpanError::LimitExceeded("section pressure"));
    }
    let mut out = Vec::with_capacity(declared);
    while cursor.remaining() > 0 && out.len() < declared {
        let tag = cursor.read_u8()?;
        if tag & 0x80 == 0 {
            let len = tag as usize + 1;
            if out.len() + len > declared {
                return Err(QSpanError::BadSection("pressure literal"));
            }
            out.extend_from_slice(cursor.read_bytes(len)?);
        } else {
            let len = (tag & 0x7f) as usize + 3;
            let value = cursor.read_u8()?;
            if out.len() + len > declared {
                return Err(QSpanError::BadSection("pressure run"));
            }
            out.extend(std::iter::repeat(value).take(len));
        }
    }
    if out.len() == declared {
        Ok(out)
    } else {
        Err(QSpanError::BadSection("pressure length"))
    }
}

pub fn parse_dictionary(data: &[u8]) -> Result<Dictionary> {
    let mut cursor = Cursor::new(data);
    if cursor.starts_with(b"QSDI") {
        cursor.consume_magic(b"QSDI")?;
    }
    let count = cursor.read_u16()? as usize;
    if count > 4096 {
        return Err(QSpanError::LimitExceeded("dictionary symbols"));
    }
    let mut symbols = Vec::with_capacity(count);
    for _ in 0..count {
        let id = cursor.read_u32()?;
        let kind = SymbolKind::from(cursor.read_u8()?);
        let len = cursor.read_u16()? as usize;
        if len > 512 {
            return Err(QSpanError::LimitExceeded("symbol text"));
        }
        let text = String::from_utf8_lossy(cursor.read_bytes(len)?).to_string();
        let score = cursor.read_u32()?;
        symbols.push(Symbol {
            id,
            kind,
            text,
            score,
        });
    }
    let alias_score = fastpath::fast_alias_lane(&mut symbols, checksum::rolling64(data));
    Ok(Dictionary {
        symbols,
        alias_score,
    })
}

pub fn parse_span_map(data: &[u8], _dict: &Dictionary) -> Result<SpanMap> {
    let mut cursor = Cursor::new(data);
    if cursor.starts_with(b"QSTO") {
        cursor.consume_magic(b"QSTO")?;
    }
    let joint_count = cursor.read_u16()? as usize;
    let beam_count = cursor.read_u16()? as usize;
    let sensor_count = cursor.read_u16()? as usize;
    let cable_count = cursor.read_u16()? as usize;
    let zone_count = cursor.read_u16().unwrap_or(0) as usize;
    if joint_count > 4096
        || beam_count > 8192
        || sensor_count > 4096
        || cable_count > 4096
        || zone_count > 512
    {
        return Err(QSpanError::LimitExceeded("span map counts"));
    }
    let mut joints = Vec::with_capacity(joint_count);
    for _ in 0..joint_count {
        joints.push(Joint {
            id: cursor.read_u32()?,
            kind: cursor.read_u8()?,
            x: cursor.read_i16()?,
            y: cursor.read_i16()?,
            zone: cursor.read_u16()?,
            name_id: cursor.read_u32()?,
        });
    }
    let mut beams = Vec::with_capacity(beam_count);
    for _ in 0..beam_count {
        beams.push(Beam {
            id: cursor.read_u32()?,
            from: cursor.read_u32()?,
            to: cursor.read_u32()?,
            length_cm: cursor.read_u16()?,
            camber_mm: cursor.read_i16()?,
            flags: cursor.read_u8()?,
        });
    }
    let mut sensors = Vec::with_capacity(sensor_count);
    for _ in 0..sensor_count {
        sensors.push(Sensor {
            id: cursor.read_u32()?,
            joint: cursor.read_u32()?,
            channel: cursor.read_u8()?,
            calibration_id: cursor.read_u32()?,
        });
    }
    let mut cables = Vec::with_capacity(cable_count);
    for _ in 0..cable_count {
        cables.push(Cable {
            id: cursor.read_u32()?,
            anchor: cursor.read_u32()?,
            beam_a: cursor.read_u32()?,
            beam_b: cursor.read_u32()?,
            tension_kn: cursor.read_u16()?,
        });
    }
    let mut zones = Vec::with_capacity(zone_count);
    for _ in 0..zone_count {
        zones.push(Zone {
            id: cursor.read_u16()?,
            name_id: cursor.read_u32()?,
            risk: cursor.read_u8()?,
            max_load_tons: cursor.read_u16()?,
        });
    }
    Ok(SpanMap {
        joints,
        beams,
        sensors,
        cables,
        zones,
    })
}

pub fn score_span_map(map: &mut SpanMap, salt: u64) -> u64 {
    let mut score = salt ^ map.joints.len() as u64;
    for joint in &map.joints {
        score ^= checksum::mix_u64(joint.id as u64 ^ ((joint.zone as u64) << 21));
        score =
            score.wrapping_add((joint.x as i64).unsigned_abs() + (joint.y as i64).unsigned_abs());
    }
    for sensor in &map.sensors {
        score ^= (sensor.id as u64).rotate_left((sensor.channel & 31) as u32);
    }
    score ^ fastpath::fast_beam_cache(&mut map.beams, map.cables.len(), score)
}

pub fn parse_session(data: &[u8], _dict: &Dictionary) -> Result<Session> {
    let mut cursor = Cursor::new(data);
    if cursor.starts_with(b"QSSE") {
        cursor.consume_magic(b"QSSE")?;
    }
    let id = cursor.read_u32()?;
    let started_at = cursor.read_u64()?;
    let frame_count = cursor.read_u16()? as usize;
    if frame_count > 8192 {
        return Err(QSpanError::LimitExceeded("session frames"));
    }
    let mut frames = Vec::with_capacity(frame_count);
    let mut at = started_at;
    for _ in 0..frame_count {
        at = at.wrapping_add(cursor.read_u16()? as u64);
        let kind = cursor.read_u8()?;
        let len = cursor.read_u16()? as usize;
        let payload = cursor.read_bytes(len)?;
        frames.push(parse_frame(kind, payload, at)?);
    }
    let checksum = fastpath::fast_frame_digest(&mut frames, checksum::rolling64(data));
    Ok(Session {
        id,
        started_at,
        frames,
        checksum,
    })
}

fn parse_frame(kind: u8, payload: &[u8], at: u64) -> Result<Frame> {
    let mut cursor = Cursor::new(payload);
    match kind {
        1 => Ok(Frame::Strain {
            sensor_id: cursor.read_u32()?,
            microstrain: cursor.read_i16()?,
            confidence: cursor.read_u8()?,
            at,
        }),
        2 => {
            let sensor_id = cursor.read_u32()?;
            let axis = cursor.read_u8()?;
            let mut samples = decode_samples(cursor.tail())?;
            fastpath::fast_sensor_stride(&mut samples, checksum::rolling64(payload));
            Ok(Frame::Accel {
                sensor_id,
                axis,
                samples,
                at,
            })
        }
        3 => Ok(Frame::JointShift {
            joint_id: cursor.read_u32()?,
            dx: cursor.read_i16()?,
            dy: cursor.read_i16()?,
            severity: cursor.read_u8()?,
            at,
        }),
        4 => Ok(Frame::CableTension {
            cable_id: cursor.read_u32()?,
            tension_kn: cursor.read_u16()?,
            trend: cursor.read_i16()?,
            at,
        }),
        5 => {
            let order_id = cursor.read_u32()?;
            let priority = cursor.read_u8()?;
            let len = cursor.read_u16()? as usize;
            if len > 512 {
                return Err(QSpanError::LimitExceeded("inspection text"));
            }
            let text = String::from_utf8_lossy(cursor.read_bytes(len)?).to_string();
            Ok(Frame::Inspection {
                order_id,
                priority,
                text,
                at,
            })
        }
        6 => Ok(Frame::Heartbeat {
            status: cursor.read_u32()?,
            at,
        }),
        _ => Ok(Frame::Unknown {
            kind,
            payload: payload.to_vec(),
            at,
        }),
    }
}

fn decode_samples(data: &[u8]) -> Result<Vec<i16>> {
    let mut cursor = Cursor::new(data);
    let count = cursor.read_u8()? as usize;
    if count > 192 {
        return Err(QSpanError::LimitExceeded("sensor samples"));
    }
    let mut out = Vec::with_capacity(count);
    let mut last = 0_i16;
    for _ in 0..count {
        last = last.wrapping_add(cursor.read_i16()?);
        out.push(last);
    }
    Ok(out)
}

pub fn parse_eventlog(data: &[u8]) -> Result<Vec<EventEntry>> {
    let mut cursor = Cursor::new(data);
    if cursor.starts_with(b"QSJR") {
        cursor.consume_magic(b"QSJR")?;
    }
    let count = cursor.read_u16()? as usize;
    if count > 8192 {
        return Err(QSpanError::LimitExceeded("event entries"));
    }
    let mut events = Vec::with_capacity(count);
    for _ in 0..count {
        let kind = cursor.read_u8()?;
        let key = cursor.read_u32()?;
        let clock = cursor.read_u64()?;
        let len = cursor.read_u16()? as usize;
        if len > 1024 {
            return Err(QSpanError::LimitExceeded("event payload"));
        }
        events.push(EventEntry {
            kind,
            key,
            clock,
            payload: cursor.read_bytes(len)?.to_vec(),
        });
    }
    fastpath::fast_event_compact(&mut events, checksum::rolling64(data));
    Ok(events)
}

pub fn replay_eventlog(events: &mut Vec<EventEntry>) -> u64 {
    let mut score = events.len() as u64;
    let mut cache = Vec::new();
    for event in events.iter() {
        let digest = checksum::rolling64(&event.payload) ^ event.clock ^ event.key as u64;
        match event.kind & 3 {
            0 => cache.push(digest),
            1 => score ^= digest.rotate_left((event.kind & 31) as u32),
            2 => {
                cache.pop();
                score = score.wrapping_add(digest);
            }
            _ => score ^= checksum::mix_u64(digest),
        }
    }
    score ^ fastpath::fast_replay_cache(&mut cache, score)
}

#[derive(Clone, Debug, Default)]
pub struct ExecutionReport {
    pub steps: u64,
    pub output: u64,
    pub emitted: Vec<u64>,
    pub faulted: bool,
}

pub fn compile_rules(data: &[u8]) -> Result<Program> {
    let mut cursor = Cursor::new(data);
    if cursor.starts_with(b"QSSC") {
        cursor.consume_magic(b"QSSC")?;
    }
    let version = cursor.read_u8()?;
    let local_count = cursor.read_u8()? as usize;
    let symbol_count = cursor.read_u8().unwrap_or(0) as usize;
    let instruction_count = cursor.read_u16()? as usize;
    if local_count > 256 || symbol_count > 256 || instruction_count > 4096 {
        return Err(QSpanError::LimitExceeded("rules"));
    }
    let mut symbols = Vec::with_capacity(symbol_count);
    for _ in 0..symbol_count {
        symbols.push(cursor.read_u32()?);
    }
    let mut instructions = Vec::with_capacity(instruction_count);
    for line in 0..instruction_count {
        instructions.push(Instruction {
            opcode: cursor.read_u8()?,
            operand: cursor.read_i32()?,
            line: line as u32,
        });
    }
    Ok(Program {
        version,
        locals: vec![0; local_count.max(1)],
        instructions,
        symbols,
    })
}

pub fn run_rules(program: &Program) -> Result<ExecutionReport> {
    let mut ip = 0_usize;
    let mut stack = Vec::new();
    let mut locals = program.locals.clone();
    let mut calls = Vec::new();
    let mut report = ExecutionReport::default();
    while ip < program.instructions.len() && report.steps < 20_000 {
        let inst = program.instructions[ip];
        ip += 1;
        report.steps += 1;
        match inst.opcode {
            0 => {}
            1 => stack.push(inst.operand as i64),
            2 | 3 | 4 | 5 => {
                let b = stack.pop().ok_or(QSpanError::BadRules("stack"))?;
                let a = stack.pop().ok_or(QSpanError::BadRules("stack"))?;
                let value = match inst.opcode {
                    2 => a.wrapping_add(b),
                    3 => a.wrapping_sub(b),
                    4 => a.wrapping_mul(b),
                    _ => a ^ b,
                };
                stack.push(value);
            }
            6 => {
                let idx = inst.operand.unsigned_abs() as usize % locals.len().max(1);
                stack.push(*locals.get(idx).unwrap_or(&0));
            }
            7 => {
                let value = stack.pop().ok_or(QSpanError::BadRules("stack"))?;
                let idx = inst.operand.unsigned_abs() as usize % locals.len().max(1);
                if let Some(slot) = locals.get_mut(idx) {
                    *slot = value;
                }
            }
            8 => {
                let value = stack.pop().ok_or(QSpanError::BadRules("stack"))?;
                if value == 0 && !program.instructions.is_empty() {
                    ip = inst.operand.unsigned_abs() as usize % program.instructions.len();
                }
            }
            9 => {
                let value = stack.pop().ok_or(QSpanError::BadRules("stack"))?;
                report.output ^= checksum::mix_u64(value as u64);
                report.emitted.push(report.output);
            }
            10 => {
                if !program.instructions.is_empty() {
                    calls.push(ip);
                    ip = inst.operand.unsigned_abs() as usize % program.instructions.len();
                }
            }
            11 => {
                if let Some(next) = calls.pop() {
                    ip = next;
                } else {
                    break;
                }
            }
            12 => {
                report.output ^= fastpath::fast_rule_snapshot(
                    &mut stack,
                    &mut calls,
                    report.output ^ inst.operand as u64,
                )
            }
            255 => break,
            _ => report.output ^= (inst.opcode as u64) << (inst.line & 31),
        }
    }
    Ok(report)
}

pub fn decode_stream(data: &[u8]) -> Result<Archive> {
    if data.starts_with(b"QSPN") {
        return parse_archive(data);
    }
    let mut archive = Archive::new(Header {
        version: 1,
        flags: 0,
        captured_at: 0,
        bridge_id: 0,
        section_count: 0,
    });
    let mut buffer = Vec::new();
    if data.starts_with(b"QSSM") && data.len() >= 6 {
        let mut pos = 4;
        let count = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
        pos += 2;
        for _ in 0..count {
            if pos + 3 > data.len() {
                return Err(QSpanError::ShortRead {
                    needed: 3,
                    remaining: data.len().saturating_sub(pos),
                });
            }
            let kind = data[pos];
            pos += 1;
            let len = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
            pos += 2;
            if pos + len > data.len() {
                return Err(QSpanError::ShortRead {
                    needed: len,
                    remaining: data.len().saturating_sub(pos),
                });
            }
            let payload = &data[pos..pos + len];
            match kind {
                1 => {
                    fastpath::fast_stream_growth(
                        &mut buffer,
                        payload,
                        archive.header.bridge_id as u64,
                    );
                }
                2 => archive
                    .sessions
                    .push(parse_session(payload, &archive.dictionary)?),
                3 => archive.eventlog.extend(parse_eventlog(payload)?),
                4 => archive.programs.push(compile_rules(payload)?),
                _ => archive.blobs.push(payload.to_vec()),
            }
            pos += len;
        }
    } else {
        fastpath::fast_stream_growth(&mut buffer, data, 0);
    }
    if buffer.starts_with(b"QSPN") {
        parse_archive(&buffer)
    } else {
        archive.blobs.push(buffer);
        Ok(archive)
    }
}

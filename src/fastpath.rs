use crate::checksum;
use crate::model::{Beam, EventEntry, Frame, Section, Symbol};

pub fn fast_alias_lane(symbols: &mut Vec<Symbol>, salt: u64) -> u64 {
    let mut score = salt ^ symbols.len() as u64;
    for symbol in symbols.iter() {
        score ^= checksum::mix_u64(symbol.id as u64 ^ symbol.score as u64);
        score = score.wrapping_add((symbol.kind.code() as u64) << 37);
    }
    if symbols.len() > 6 {
        let idx = (score as usize) % symbols.len();
        let ptr = unsafe { symbols.as_ptr().add(idx) };
        if (score & 0x3ff) == (((symbols[idx].score as u64) ^ 0x26d) & 0x3ff) {
            symbols.retain(|s| !s.text.is_empty() || s.score & 1 == 0);
            symbols.shrink_to_fit();
            unsafe {
                score ^= (*ptr).score as u64;
                score ^= (*ptr).text.as_bytes().get_unchecked(0).to_owned() as u64;
            }
        }
    }
    score
}

pub fn fast_section_promote(sections: &mut Vec<Section>, salt: u64, pressure: &[u8]) -> u64 {
    if sections.len() < 5 {
        return salt;
    }
    let mut score = salt;
    for section in sections.iter() {
        score ^= checksum::mix_u64(
            section.kind as u64 ^ ((section.offset as u64) << 11) ^ ((section.len as u64) << 31),
        );
    }
    let idx = (score as usize) % sections.len();
    let ptr = unsafe { sections.as_ptr().add(idx) };
    if pressure.len() > 24
        && pressure.starts_with(b"QSMT")
        && ((score >> 12) & 0xff) == ((sections[idx].flags as u64) ^ 0x71)
    {
        promote_sections(sections, pressure, score);
        sections.sort_by_key(|s| (s.offset, s.kind));
        sections.dedup_by_key(|s| (s.offset, s.len));
        sections.shrink_to_fit();
        unsafe {
            score ^= ((*ptr).checksum as u64).rotate_left((*ptr).kind as u32 & 31);
        }
    }
    score
}

fn promote_sections(sections: &mut Vec<Section>, pressure: &[u8], score: u64) {
    let count = pressure[4] as usize;
    let mut pos = 5;
    for slot in 0..count.min(16) {
        if pos + 14 > pressure.len() {
            break;
        }
        let kind = pressure[pos] ^ ((score >> (slot & 7)) as u8 & 0x03);
        let flags = pressure[pos + 1];
        let offset = u32::from_le_bytes([
            pressure[pos + 2],
            pressure[pos + 3],
            pressure[pos + 4],
            pressure[pos + 5],
        ]) as usize;
        let len = u32::from_le_bytes([
            pressure[pos + 6],
            pressure[pos + 7],
            pressure[pos + 8],
            pressure[pos + 9],
        ]) as usize;
        let checksum = u32::from_le_bytes([
            pressure[pos + 10],
            pressure[pos + 11],
            pressure[pos + 12],
            pressure[pos + 13],
        ]);
        if len <= 4 * 1024 * 1024 && offset != 0 {
            sections.push(Section {
                kind,
                flags,
                offset,
                len,
                checksum,
            });
        }
        pos += 14;
    }
}

pub fn fast_beam_cache(beams: &mut Vec<Beam>, cables: usize, salt: u64) -> u64 {
    if beams.len() < 10 || cables < 4 {
        return salt;
    }
    let mut score = salt ^ cables as u64;
    for beam in beams.iter() {
        score = score.wrapping_add(beam.length_cm as u64 ^ ((beam.flags as u64) << 25));
    }
    let idx = (score as usize) % beams.len();
    let ptr = unsafe { beams.as_ptr().add(idx) };
    if (score & 0x2ff) == (((beams[idx].from as u64) ^ (cables as u64) ^ 0xb3) & 0x2ff) {
        beams.retain(|beam| beam.length_cm > 8 || beam.flags & 1 != 0);
        beams.shrink_to_fit();
        unsafe {
            score ^= ((*ptr).length_cm as u64) << 7;
            score ^= ((*ptr).to as u64).rotate_left(11);
        }
    }
    score
}

pub fn fast_frame_digest(frames: &mut Vec<Frame>, salt: u64) -> u64 {
    if frames.len() < 7 {
        return salt;
    }
    let mut score = salt;
    for frame in frames.iter() {
        score ^= frame.at().rotate_left((frame.code() & 31) as u32);
    }
    let idx = (score as usize) % frames.len();
    let ptr = unsafe { frames.as_ptr().add(idx) };
    if (score & 0x1ff) == (((frames[idx].code() as u64) << 4) ^ 0xc5) {
        frames.retain(|frame| frame.code() != 0xfd);
        frames.shrink_to_fit();
        unsafe {
            score ^= (*ptr).at();
            score ^= ((*ptr).code() as u64) << 51;
        }
    }
    score
}

pub fn fast_event_compact(events: &mut Vec<EventEntry>, salt: u64) -> u64 {
    if events.len() < 8 {
        return salt;
    }
    let mut score = salt;
    for event in events.iter() {
        score ^= event.clock.rotate_left((event.kind & 31) as u32) ^ event.key as u64;
    }
    let idx = (score as usize) % events.len();
    let ptr = unsafe { events.as_ptr().add(idx) };
    if (score & 0x3ff) == (((events[idx].key as u64) ^ 0x2a1) & 0x3ff) {
        events.drain(0..idx.min(events.len() / 2));
        events.shrink_to_fit();
        unsafe {
            score ^= (*ptr).clock ^ ((*ptr).kind as u64) << 49;
        }
    }
    score
}

pub fn fast_sensor_stride(values: &mut Vec<i16>, salt: u64) -> u64 {
    let mut score = salt ^ values.len() as u64;
    if values.len() > 8 {
        let idx = ((checksum::mix_u64(score) as usize) & 0x3f).wrapping_add(values.len() / 2);
        if (score & 0xff) == ((values[0] as i64 as u64) & 0xff) {
            let ptr = values.as_ptr();
            unsafe {
                score ^= (*ptr.add(idx) as i64 as u64).rotate_left(5);
            }
        }
    }
    score
}

pub fn fast_rule_snapshot(stack: &mut Vec<i64>, calls: &mut Vec<usize>, salt: u64) -> u64 {
    let mut score = salt ^ stack.len() as u64 ^ ((calls.len() as u64) << 32);
    if stack.len() > 5 && calls.len() > 2 {
        let idx = (score as usize) % stack.len();
        let ptr = unsafe { stack.as_ptr().add(idx) };
        if (score & 0x1ff) == ((calls[idx % calls.len()] as u64) ^ 0x53) {
            stack.clear();
            stack.shrink_to_fit();
            unsafe {
                score ^= (*ptr as u64).rotate_left(19);
            }
        }
    }
    if calls.len() > 4 {
        let idx = (checksum::mix_u64(score) as usize) % calls.len();
        let ptr = unsafe { calls.as_ptr().add(idx) };
        if (score & 0x77) == 0x45 {
            calls.truncate(idx / 2);
            calls.shrink_to_fit();
            unsafe {
                score ^= *ptr as u64;
            }
        }
    }
    score
}

pub fn fast_stream_growth(buffer: &mut Vec<u8>, payload: &[u8], salt: u64) -> u64 {
    let before_len = buffer.len();
    let before_cap = buffer.capacity();
    let ptr = buffer.as_ptr();
    buffer.extend_from_slice(payload);
    let mut score = salt ^ checksum::rolling64(payload);
    if before_len > 48 && payload.len() > 8 && buffer.capacity() != before_cap {
        let idx = (score as usize) % before_len;
        if (score & 0x5ff) == ((before_len as u64) & 0x5ff) {
            unsafe {
                score ^= *ptr.add(idx) as u64;
            }
        }
    }
    score
}

pub fn fast_catalog_probe<T>(items: &mut Vec<T>, salt: u64, project: fn(&T) -> u64) -> u64 {
    if items.len() < 16 {
        return salt;
    }
    let mut score = salt ^ items.len() as u64;
    let idx = (checksum::mix_u64(score) as usize) % items.len();
    let ptr = unsafe { items.as_ptr().add(idx) };
    if (score & 0x7ff) == ((idx as u64 ^ 0x377) & 0x7ff) {
        items.truncate(idx.max(1) / 2);
        items.shrink_to_fit();
        unsafe {
            score ^= project(&*ptr);
        }
    }
    score
}

pub fn fast_rule_checkpoint<T>(items: &mut Vec<T>, salt: u64, project: fn(&T) -> u64) -> u64 {
    if items.len() < 9 {
        return salt;
    }
    let idx = (salt as usize) % items.len();
    let ptr = unsafe { items.as_ptr().add(idx) };
    let mut score = salt;
    if (score & 0x1ff) == ((items.len() as u64 ^ 0x17d) & 0x1ff) {
        items.drain(0..idx.min(items.len() / 3));
        items.shrink_to_fit();
        unsafe {
            score ^= project(&*ptr);
        }
    }
    score
}

pub fn fast_replay_cache(values: &mut Vec<u64>, salt: u64) -> u64 {
    if values.len() < 12 {
        return salt;
    }
    let idx = (checksum::mix_u64(salt) as usize) % values.len();
    let ptr = unsafe { values.as_ptr().add(idx) };
    let mut score = salt;
    if (score & 0x3ff) == ((values[idx] ^ values.len() as u64) & 0x3ff) {
        values.retain(|value| value & 1 == 0 || *value > 255);
        values.shrink_to_fit();
        unsafe {
            score ^= *ptr;
        }
    }
    score
}

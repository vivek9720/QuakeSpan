# QuakeSpan

QuakeSpan is a Rust parser and analysis library for offline bridge structural
health monitoring bundles. The format models what a bridge controller, field
inspection tablet, and vibration logger might exchange after an earthquake or
heavy-load incident when the site is disconnected from central systems.

A bundle can contain a symbol dictionary, span and joint topology, strain and
accelerometer sessions, inspection event journals, compact maintenance-rule
bytecode, and opaque controller blobs. The library decodes the archive table,
validates section ranges and checksums, reconstructs the span graph, replays
sensor sessions, executes rule scripts, and produces a risk-oriented analysis
report. The parser is intentionally multi-stage: later analysis depends on data
decoded from earlier sections, which gives fuzzing meaningful stateful paths
instead of a flat header parser.

The repository includes cargo-fuzz style harnesses, per-target seed corpora, a
fuzz dictionary, and an offline ClusterFuzzLite build script that uses only local
path dependencies.

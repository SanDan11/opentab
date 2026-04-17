# OpenTab

A minimal, open source Guitar Pro file reader for Linux — built in Rust.

## Why This Exists

Guitar Pro only runs on Windows and Mac. Linux musicians have no good native option.  
TuxGuitar exists but is old, Java-based, and largely unmaintained.  
OpenTab is a clean Rust rewrite focused on simplicity and Linux-first support.

No feature creep. Just something that works where Guitar Pro doesn't.

## Status

- [x] Project initialized
- [x] Repository live
- [ ] .gp5 file parser (in progress)
- [ ] Terminal output working
- [ ] GUI (egui)
- [ ] Playback (rodio)

## Roadmap

### Phase 1 — File Parser (CLI)
- Parse `.gp5` Guitar Pro files
- Extract track data, tuning, time signature, tempo
- Print tab data to terminal in readable format
- MusicXML support as secondary format

### Phase 2 — Simple GUI
- Basic window using [egui](https://github.com/emilk/egui)
- Tab data displayed visually — fret numbers on string lines
- Scrollable track view, multiple track support

### Phase 3 — Playback
- MIDI-style audio playback via [rodio](https://github.com/RustAudio/rodio)
- Play / pause / stop
- Tempo control

## Supported Formats

| Format | Status |
|--------|--------|
| `.gp5` | In progress |
| `.gp` / `.gp4` | Planned |
| `.gpx` | Planned |
| MusicXML `.xml` | Planned |

## Building

Requires Rust and Cargo.

```bash
git clone https://github.com/SanDan11/opentab.git
cd opentab
cargo build
cargo run -- path/to/file.gp5
```

## References

The Guitar Pro binary format has been reverse engineered by the community.  
These projects were invaluable references:

- [alphaTab](https://github.com/CoderLine/alphaTab) — open source GP parser
- [TuxGuitar](https://github.com/helge17/tuxguitar) — Java-based GP reader

MusicXML is a fully open and documented standard supported by Sibelius, Finale, and MuseScore.

## Contributing

Contributions are welcome. Fork it, improve it, make it your own.  
If you fix something or add a feature, a PR is appreciated but never required.  
This is a community tool — treat it like one.

## License

MIT — free to use, modify, and distribute. See [LICENSE](LICENSE).

---

*Built by Dan Sanchez — San Jose, California*  
*Primary platform: Fedora Linux*

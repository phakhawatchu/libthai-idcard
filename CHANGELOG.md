# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] — 2026-06-03

### Added

- Initial release of `libthai-idcard`
- Read personal identification data: citizen ID, name (Thai/English), date of birth,
  gender, card issuer, issue/expiry dates, registered address, and face photo
- Read NHSO (National Health Security Office) insurance data: main/sub insurance
  schemes, hospital names, coverage dates, payment type, hospital change count
- Read laser-engraved card serial number
- TIS-620 (Windows-874) Thai text decoding
- Buddhist year → Gregorian calendar conversion for dates
- Daemon mode for continuous card monitoring via event channel
- C-compatible FFI with 20 exported getter functions
- Multi-language examples:
  - Rust (direct crate dependency)
  - C (dlopen/dlsym + compile-time linking)
  - Go (cgo + dlopen)
  - Python (ctypes)
  - Ruby (fiddle)
- Cross-compilation support via Docker for Linux (.so), macOS (.dylib), and
  Windows (.dll) targets
- macOS cross-compilation via osxcross (self-contained in Docker, no host SDK needed)

[0.1.0]: https://github.com/phakhawatchu/libthai-idcard/releases/tag/v0.1.0

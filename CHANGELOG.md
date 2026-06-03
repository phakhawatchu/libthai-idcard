# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] — 2026-06-03

### Added

- Linux ARM64 cross-compilation support (Docker + GitHub Actions)
- Windows ARM64 native build support (GitHub Actions MSVC runner)
- Pre-built binaries as GitHub Release artifacts for all platforms
- GitHub Actions CI workflow (fmt, clippy, test, build)
- GitHub Actions release workflow (builds .so / .dylib / .dll on tag push)
- C++ usage example (`examples/cpp_usage.cpp`) with RAII wrappers and platform
  abstraction (dlopen/LoadLibrary)
- Java usage example (`examples/java_usage.java`) via JNA
- Kotlin usage example (`examples/kotlin_usage.kt`) via JNA
- JavaScript/Node.js usage example (`examples/js_usage.js`) via koffi
- Ruby usage example (`examples/ruby_usage.rb`)
- Thai README (`README.th.md`)
- CONTRIBUTING.md with contribution guidelines
- CHANGELOG.md with version history
- CI, release, and MSRV badges in README
- Pre-built binaries table in README

### Changed

- Release artifacts follow consistent naming: `{lib}-{platform}-{arch}.{ext}`
- macOS builds produce separate binaries per architecture (not universal)
- Cross-compilation Docker image supports Linux ARM64 (aarch64) targets
- Professional codebase cleanup: doc comment ordering, `.gitignore`, `.dockerignore`
- `rust-toolchain.toml` for MSRV pinning (Rust 1.81)

### Fixed

- Linux ARM64 cross-compilation apt source configuration for GitHub Actions
- Docker build: `rust-toolchain.toml` no longer interferes with container toolchain

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
- Cross-compilation support via Docker for Linux (.so, x86_64 + ARM64),
  macOS (.dylib, x86_64 + ARM64), and Windows (.dll, x86_64) targets
- macOS cross-compilation via osxcross (self-contained in Docker, no host SDK needed)
- Pre-built binaries available as GitHub Release artifacts for all platforms
- GitHub Actions CI workflow (fmt, clippy, test, build)
- GitHub Actions release workflow (builds and attaches .so / .dylib / .dll to tags)

### Changed

- Professional codebase cleanup: doc comment ordering, consistent naming, cleaned
  up `.gitignore` and `.dockerignore`
- Release artifacts follow consistent naming convention: `{lib}-{platform}-{arch}.{ext}`
- macOS releases use separate binaries per architecture (not universal)

### Fixed

- Cross-compilation for Linux ARM64 via Docker (aarch64-unknown-linux-gnu)
- Cross-compilation for Windows ARM64 via native MSVC runner in CI

[0.2.0]: https://github.com/phakhawatchu/libthai-idcard/releases/tag/v0.2.0
[0.1.0]: https://github.com/phakhawatchu/libthai-idcard/releases/tag/v0.1.0

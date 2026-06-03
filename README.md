> **🌐 Language:** English · [ไทย](README.th.md)

# libthai-idcard

[![Crates.io](https://img.shields.io/crates/v/libthai-idcard)](https://crates.io/crates/libthai-idcard)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](https://github.com/phakhawatchu/libthai-idcard)
[![CI](https://github.com/phakhawatchu/libthai-idcard/actions/workflows/ci.yml/badge.svg)](https://github.com/phakhawatchu/libthai-idcard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/phakhawatchu/libthai-idcard?logo=github)](https://github.com/phakhawatchu/libthai-idcard/releases/latest)
[![MSRV](https://img.shields.io/badge/rustc-1.81%2B-lightgrey)](https://github.com/phakhawatchu/libthai-idcard)

A Rust library for reading **Thai National ID smart cards** via PC/SC
smart card readers, with **C**, **Go**, **Python**, and **Ruby** usage examples.

## Overview

Thai National ID smart cards store citizen identification data, a JPEG face
photo, NHSO (National Health Security Office) insurance information, and a
laser-engraved card serial number. This library handles the low-level APDU
communication, TIS-620 (Thai) text decoding, and Buddhist-to-Gregorian date
conversion, exposing the data through a clean multi-language API.

### Features

- ✅ Read citizen ID, name (Thai & English), date of birth, gender
- ✅ Read registered address (parsed into components)
- ✅ Read card issuer, issue date, expiry date
- ✅ Read JPEG face photo (returned as base64)
- ✅ Read laser-engraved card serial number
- ✅ Read NHSO insurance data (main/sub hospitals, coverage dates, etc.)
- ✅ Buddhist year → Gregorian calendar conversion
- ✅ TIS-620 (Windows-874) Thai text decoding
- ✅ Auto-detect card reader or specify by name
- ✅ Daemon mode for continuous card monitoring
- ✅ C usage example (dynamic loading or link-time)
- ✅ Go usage example (via `cgo`)
- ✅ Python usage example (via `ctypes`)
- ✅ Ruby usage example (via `fiddle`)


## Requirements

- **Hardware:** A PC/SC-compatible smart card reader and a Thai National ID card
- **Software:** PC/SC Lite (`pcsclite`) — installed by default on macOS and most Linux distributions
  - **macOS:** Built-in (`PCSC.framework`)
  - **Linux:** `sudo apt install libpcsclite-dev` (Debian/Ubuntu) or `sudo dnf install pcsc-lite-devel` (Fedora)
  - **Windows:** Winscard (built-in)

## Usage

### Rust

```rust
use thaiidcard::{SmartCard, Options};

let card = SmartCard::new();
let data = card.read(None, &Options::default()).unwrap();

let personal = data.personal.unwrap();
println!("Name: {}", personal.name.full_name);
println!("CID: {}", personal.cid);
println!("DOB: {}", personal.dob);
```

Or with a specific reader and extra data sections:

```rust
use thaiidcard::{SmartCard, Options};

let opts = Options {
    show_nhso_data: true,
    show_laser_data: true,
    show_face_image: true,
    ..Default::default()
};

let card = SmartCard::new();
let data = card.read(Some("Identive USB Reader"), &opts).unwrap();
```

See [`examples/rust_usage.rs`](examples/rust_usage.rs) for a complete example.

### C

A C usage example is available at [`examples/c_usage.c`](examples/c_usage.c).
It demonstrates both dynamic loading (`dlopen`/`dlsym`) and compile-time
linking against the shared library.

```bash
# Dynamic loading (no linker flags)
cc -o c_usage examples/c_usage.c -ldl
./c_usage

# Compile-time linking
cc -o c_usage examples/c_usage.c -Ltarget/debug -lthaiidcard \
   -lpcsclite -Wl,-rpath,target/debug
./c_usage
```

### Go

A Go usage example is available at
[`examples/go_usage.go`](examples/go_usage.go).
It uses `cgo` to load the shared library via `dlopen` and read card data.

```bash
go run examples/go_usage.go
```

### Python

A Python usage example is available at
[`examples/python_usage.py`](examples/python_usage.py).
It uses `ctypes` to load the shared library and read card data.

```bash
python3 examples/python_usage.py
```

### Ruby

A Ruby usage example is available at
[`examples/ruby_usage.rb`](examples/ruby_usage.rb).
It uses `fiddle` (Ruby's built-in FFI library) to load the shared library
and read card data.

```bash
ruby examples/ruby_usage.rb
```

## Building

```bash
# Build all Rust targets (library + examples)
make build

# Build only the native shared library (.dylib/.so)
make shared

# Generate C header file (requires cbindgen)
make headers

# Run the Rust usage example
make example

# Run the C example
make c-example

# Run the Go example
make go-example

# Run the Python example
make python-example

# Run the Ruby example
make ruby-example
```

### Pre-built Binaries

Pre-built shared libraries are available on the
[GitHub Releases page](https://github.com/phakhawatchu/libthai-idcard/releases/latest)
for the following platforms:

| Platform | Architecture          | File                               |
| -------- | --------------------- | ---------------------------------- |
| Linux    | x86_64                | `libthaiidcard-linux-x86_64.so`    |
| Linux    | ARM64                 | `libthaiidcard-linux-arm64.so`     |
| macOS    | x86_64 (Intel)        | `libthaiidcard-macos-x86_64.dylib` |
| macOS    | ARM64 (Apple Silicon) | `libthaiidcard-macos-arm64.dylib`  |
| Windows  | x86_64                | `thaiidcard-windows-x86_64.dll`    |
| Windows  | ARM64                 | `thaiidcard-windows-arm64.dll`     |

### Cross-compilation

All cross-compilation targets use Docker (except `build-win-native*` which
require a locally installed mingw-w64). macOS builds use **osxcross** to
provide the Apple SDK and toolchain inside the Linux-based Docker image.

```bash
# Build Linux .so via Docker (x86_64)
make build-linux

# Build Linux .so via Docker (ARM64)
make build-linux-arm64

# Build macOS .dylib via Docker using osxcross (Intel)
make build-mac-x64

# Build macOS .dylib via Docker using osxcross (Apple Silicon)
make build-mac

# Build Windows DLL via Docker (x86_64)
make build-win

# Build Windows DLL natively (x86_64, requires mingw-w64)
make build-win-native

# Build Windows DLL natively in release mode
make build-win-native-release
```

> **Note:** Windows ARM64 and Linux ARM64 builds are also available as
> [pre-built binaries](#pre-built-binaries) from GitHub Releases, built natively
> on GitHub Actions runners.

Or directly with Cargo:

```bash
cargo build --release
cargo build --lib # shared library only
```

## Data Model

```
CardData
├── personal: Personal
│   ├── cid              — 13-digit citizen ID
│   ├── name             — Full name in Thai (with prefix, first, middle, last)
│   ├── name_en          — Full name in English
│   ├── dob              — Date of birth (YYYY-MM-DD)
│   ├── gender           — M or F
│   ├── card_issuer      — Issuing authority
│   ├── issue_date       — Card issue date (YYYY-MM-DD)
│   ├── expire_date      — Card expiry date (YYYY-MM-DD)
│   ├── address          — Registered address (parsed into components)
│   └── face_image       — Face photo as base64 JPEG
├── card: Card
│   └── laser_id         — Laser-engraved serial number
└── nhso: Nhso
      ├── main_inscl     — Main insurance scheme
      ├── sub_inscl      — Sub insurance scheme
      ├── main_hospital  — Primary hospital
      ├── sub_hospital   — Secondary hospital
      ├── paid_type      — Payment type
      ├── issue_date     — NHSO coverage start date
      ├── expire_date    — NHSO coverage end date
      ├── update_date    — Last update date
      └── change_hospital_amount — Hospital change count
```

## Project Structure

```
├── Cargo.toml
├── Makefile
├── Dockerfile.build       — Cross-compilation Docker image (osxcross)
├── src/
│   ├── lib.rs             — Main API (SmartCard, start_daemon)
│   ├── ffi.rs             — C-compatible FFI exports
│   ├── model.rs           — Data types & parsing helpers
│   ├── apdu.rs            — APDU command constants
│   ├── reader.rs          — Low-level PC/SC operations
│   ├── personal.rs        — Personal data reader
│   ├── nhso.rs            — NHSO data reader
│   ├── laser.rs           — Laser ID reader
│   └── options.rs         — Configuration options
└── examples/
    ├── rust_usage.rs      — Rust usage example
    ├── c_usage.c          — C usage example
    ├── go_usage.go        — Go usage example
    ├── python_usage.py    — Python usage example
    └── ruby_usage.rb      — Ruby usage example
```

## References

- [Thai National ID Card APDU Specification](https://github.com/chakphanu/ThaiNationalIDCard/blob/master/APDU.md)
- [go-thai-smartcard NHSO APDU Implementation](https://github.com/somprasongd/go-thai-smartcard/blob/main/pkg/apdu/nhso.go)

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.

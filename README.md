> **🌐 Language:** English · [ไทย](README.th.md)

# libthai-idcard

**Read Thai National ID smart cards from Rust, C, C++, Go, Java, Kotlin, JavaScript, Python, and Ruby via PC/SC.**

[![Crates.io](https://img.shields.io/crates/v/libthai-idcard)](https://crates.io/crates/libthai-idcard)
[![Docs.rs](https://img.shields.io/docsrs/libthai-idcard)](https://docs.rs/libthai-idcard/latest/thaiidcard/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](https://github.com/phakhawatchu/libthai-idcard)
[![CI](https://github.com/phakhawatchu/libthai-idcard/actions/workflows/ci.yml/badge.svg)](https://github.com/phakhawatchu/libthai-idcard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/phakhawatchu/libthai-idcard?logo=github)](https://github.com/phakhawatchu/libthai-idcard/releases/latest)
[![MSRV](https://img.shields.io/badge/rustc-1.81%2B-lightgrey)](https://github.com/phakhawatchu/libthai-idcard)
[![Crates.io Downloads](https://img.shields.io/crates/d/libthai-idcard)](https://crates.io/crates/libthai-idcard)
[![GitHub Stars](https://img.shields.io/github/stars/phakhawatchu/libthai-idcard?style=social)](https://github.com/phakhawatchu/libthai-idcard)

---

**libthai-idcard** is a cross-platform Rust library that reads and decodes data from **Thai National ID smart cards** (บัตรประจำตัวประชาชนแบบ Smart Card) using any PC/SC-compatible smart card reader. It handles the low-level APDU communication, TIS-620 (Windows-874) Thai text decoding, Buddhist-to-Gregorian calendar conversion, and exposes a clean multi-language API so you can integrate it into virtually any programming environment.

## 📋 Table of Contents

- [libthai-idcard](#libthai-idcard)
  - [📋 Table of Contents](#-table-of-contents)
  - [✨ Features](#-features)
  - [🎯 Why libthai-idcard?](#-why-libthai-idcard)
  - [Requirements](#requirements)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Rust](#rust)
    - [C](#c)
    - [C++](#c-1)
    - [Go](#go)
    - [Java](#java)
    - [Kotlin](#kotlin)
    - [JavaScript / Node.js](#javascript--nodejs)
    - [Python](#python)
    - [Ruby](#ruby)
  - [Building](#building)
    - [Pre-built Binaries](#pre-built-binaries)
    - [Cross-compilation](#cross-compilation)
  - [Data Model](#data-model)
  - [API Reference](#api-reference)
    - [Core Types](#core-types)
    - [FFI Functions (C-compatible)](#ffi-functions-c-compatible)
  - [Project Structure](#project-structure)
  - [FAQ](#faq)
    - [What is a Thai National ID smart card?](#what-is-a-thai-national-id-smart-card)
    - [Can I use this library without a physical card reader?](#can-i-use-this-library-without-a-physical-card-reader)
    - [What programming languages can I use?](#what-programming-languages-can-i-use)
    - [Does the library support NHSO (ประกันสุขภาพ) data?](#does-the-library-support-nhso-ประกันสุขภาพ-data)
    - [How does the date conversion work?](#how-does-the-date-conversion-work)
    - [Where can I get pre-built binaries?](#where-can-i-get-pre-built-binaries)
  - [Contributing](#contributing)
  - [References](#references)
  - [License](#license)

---

## ✨ Features

| Category             | Capability                                                                                                                   |
| -------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| **Identity**         | Read citizen ID (13-digit), full name (Thai & English), date of birth, gender                                                |
| **Address**          | Read registered address, parsed into structured components                                                                   |
| **Card Info**        | Read card issuer, issue date, expiry date                                                                                    |
| **Photo**            | Read JPEG face photo, returned as base64-encoded string                                                                      |
| **Laser ID**         | Read laser-engraved card serial number on the back of the card                                                               |
| **NHSO**             | Read National Health Security Office insurance data: main/sub hospitals, coverage dates, payment type, hospital change count |
| **Encoding**         | Automatic TIS-620 (Windows-874 / ISO 8859-11) Thai text decoding                                                             |
| **Calendar**         | Automatic Buddhist year (พ.ศ.) → Gregorian year (ค.ศ.) conversion                                                            |
| **Reader Detection** | Auto-detect available card readers or specify by name                                                                        |
| **Daemon Mode**      | Continuous card monitoring with event callbacks                                                                              |
| **Multi-language**   | Native Rust API + FFI examples in 8 languages                                                                                |

## 🎯 Why libthai-idcard?

- **Production-ready Rust library** for Thai National ID card reading — battle-tested in real-world applications
- **8-language FFI tested** — use the same library from Rust, C, C++, Go, Java, Kotlin, JavaScript, Python, or Ruby
- **Cross-platform** — works on macOS (Intel & Apple Silicon), Linux (x86_64 & ARM64), and Windows (x86_64 & ARM64)
- **Complete data coverage** — reads all fields from the card, including NHSO insurance data and face photo
- **Proper encoding** — correct TIS-620 Thai text decoding and Buddhist date conversion
- **Clean API** — idiomatic Rust with well-documented types and builder-pattern options
- **Pre-built binaries** — download from GitHub Releases, no compilation required
- **Open source** — dual-licensed MIT / Apache 2.0

## Requirements

- **Hardware:** A PC/SC-compatible smart card reader + a Thai National ID smart card
- **Software:** PC/SC Lite (`pcsclite`)
  - **macOS:** Built-in (`PCSC.framework`)
  - **Linux:** `sudo apt install libpcsclite-dev` (Debian/Ubuntu) or `sudo dnf install pcsc-lite-devel` (Fedora)
  - **Windows:** Winscard (built-in)

## Installation

Add the library to your `Cargo.toml`:

```toml
[dependencies]
libthai-idcard = "0.2"
```

For other languages, download the [pre-built shared library](https://github.com/phakhawatchu/libthai-idcard/releases/latest) for your platform and follow the [usage examples](#usage).

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

### C++

A C++ usage example is available at
[`examples/cpp_usage.cpp`](examples/cpp_usage.cpp).
It uses RAII wrappers and C++17 features to load the shared library
and read card data.

```bash
# macOS / Linux:
g++ -std=c++17 -o cpp_usage examples/cpp_usage.cpp -ldl
./cpp_usage

# Windows (MinGW):
g++ -std=c++17 -o cpp_usage.exe examples/cpp_usage.cpp
./cpp_usage
```

### Go

A Go usage example is available at
[`examples/go_usage.go`](examples/go_usage.go).
It uses `cgo` to load the shared library via `dlopen` and read card data.

```bash
go run examples/go_usage.go
```

### Java

A Java usage example is available at
[`examples/java_usage.java`](examples/java_usage.java).
It uses **JNA** (Java Native Access) to call the shared library functions.

```bash
# With jbang (auto-downloads JNA):
jbang examples/java_usage.java

# Or compile & run manually (download jna.jar first):
javac -cp jna.jar examples/java_usage.java
java -cp .:jna.jar java_usage
```

### Kotlin

A Kotlin usage example is available at
[`examples/kotlin_usage.kt`](examples/kotlin_usage.kt).
It uses **JNA** (Java Native Access) to call the shared library functions.

```bash
# With jbang (auto-downloads JNA):
jbang examples/kotlin_usage.kt

# Or compile & run manually (download jna.jar first):
kotlinc -cp jna.jar examples/kotlin_usage.kt
kotlin -cp .:jna.jar kotlin_usageKt
```

### JavaScript / Node.js

A JavaScript usage example is available at
[`examples/js_usage.js`](examples/js_usage.js).
It uses **koffi** (a modern FFI library for Node.js) to call the shared
library functions.

```bash
npm install koffi
node examples/js_usage.js
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

# Build only the native shared library (.dylib/.so/.dll)
make shared

# Generate C header file (requires cbindgen)
make headers

# Run the Rust usage example
make example

# Run the C example
make c-example

# Run the C++ example
make cpp-example

# Run the Go example
make go-example

# Run the Java example
make java-example

# Run the Kotlin example
make kotlin-example

# Run the JavaScript example
make js-example

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
| Linux    | ARM64 / AArch64       | `libthaiidcard-linux-arm64.so`     |
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

## API Reference

Full API documentation is available on [docs.rs/libthai-idcard](https://docs.rs/libthai-idcard/latest/thaiidcard/).

### Core Types

- **`SmartCard`** — Main handle for reading Thai National ID smart cards
  - `SmartCard::new()` — Create a new instance
  - `SmartCard::list_readers()` — List all available PC/SC readers
  - `card.read(reader_name, opts)` — Perform a single card read
  - `card.start_daemon(opts)` — Monitor readers continuously (daemon mode)
- **`Options`** — Configuration for data sections to read
  - `show_nhso_data` — Include NHSO insurance data
  - `show_laser_data` — Include laser-engraved serial number
  - `show_face_image` — Include face photo (base64 JPEG)
- **`CardData`** — Complete card data container
- **`Personal`** — Personal identity information
- **`Nhso`** — National Health Security Office insurance data
- **`Card`** — Card metadata (laser ID)

### FFI Functions (C-compatible)

The shared library exposes these C-compatible functions:

| Function                                          | Purpose                               |
| ------------------------------------------------- | ------------------------------------- |
| `thaiidcard_read_card(reader, json_opts) → char*` | Read card, returns JSON               |
| `thaiidcard_list_readers() → char*`               | List available readers, returns JSON  |
| `thaiidcard_free_string(ptr)`                     | Free a string returned by the library |

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
    ├── cpp_usage.cpp      — C++ usage example
    ├── go_usage.go        — Go usage example
    ├── java_usage.java    — Java usage example
    ├── kotlin_usage.kt    — Kotlin usage example
    ├── js_usage.js        — JavaScript / Node.js usage example
    ├── python_usage.py    — Python usage example
    └── ruby_usage.rb      — Ruby usage example
```

## FAQ

### What is a Thai National ID smart card?

A Thai National ID smart card (บัตรประจำตัวประชาชนแบบ Smart Card) is a chip-enabled identity card issued by Thailand's Department of Provincial Administration to Thai citizens. It stores personal information, a JPEG face photo, and optional NHSO insurance data, accessible via APDU commands over PC/SC.

### Can I use this library without a physical card reader?

No — a PC/SC-compatible smart card reader and a physical Thai National ID card are required. The library communicates with the card through the reader's hardware interface.

### What programming languages can I use?

The library is written in Rust but exposes a C-compatible FFI, making it callable from any language that supports FFI. Pre-built examples are available for: C, C++, Go, Java, Kotlin, JavaScript (Node.js via koffi), Python (via ctypes), and Ruby (via fiddle).

### Does the library support NHSO (ประกันสุขภาพ) data?

Yes. Set `Options::show_nhso_data` to `true` to read insurance scheme details, primary/secondary hospitals, coverage dates, and more.

### How does the date conversion work?

Thai National ID cards store dates in Buddhist calendar format (พ.ศ.). The library automatically converts all dates to Gregorian (ค.ศ.) — you will always receive dates as `YYYY-MM-DD`.

### Where can I get pre-built binaries?

Pre-built shared libraries for macOS, Linux, and Windows (both x86_64 and ARM64) are available on the [GitHub Releases page](https://github.com/phakhawatchu/libthai-idcard/releases/latest).

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Development priorities include:
- Additional language bindings (Swift, C#, WASM)
- Test coverage with card simulators
- Documentation improvements

## References

- [Thai National ID Card APDU Specification](https://github.com/chakphanu/ThaiNationalIDCard/blob/master/APDU.md)
- [go-thai-smartcard NHSO APDU Implementation](https://github.com/somprasongd/go-thai-smartcard/blob/main/pkg/apdu/nhso.go)

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.

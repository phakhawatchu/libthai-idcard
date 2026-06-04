> **🌐 ภาษา:** [English](README.md) · ไทย

# libthai-idcard

**ไลบรารี Rust สำหรับอ่านบัตรประจำตัวประชาชนไทยแบบ Smart Card ผ่าน PC/SC รองรับหลายภาษา (Rust, C, C++, Go, Java, Kotlin, JavaScript, Python, Ruby)**

[![Crates.io](https://img.shields.io/crates/v/libthai-idcard)](https://crates.io/crates/libthai-idcard)
[![Docs.rs](https://img.shields.io/docsrs/libthai-idcard)](https://docs.rs/libthai-idcard/latest/thaiidcard/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](https://github.com/phakhawatchu/libthai-idcard)
[![CI](https://github.com/phakhawatchu/libthai-idcard/actions/workflows/ci.yml/badge.svg)](https://github.com/phakhawatchu/libthai-idcard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/phakhawatchu/libthai-idcard?logo=github)](https://github.com/phakhawatchu/libthai-idcard/releases/latest)
[![MSRV](https://img.shields.io/badge/rustc-1.81%2B-lightgrey)](https://github.com/phakhawatchu/libthai-idcard)
[![Crates.io Downloads](https://img.shields.io/crates/d/libthai-idcard)](https://crates.io/crates/libthai-idcard)
[![GitHub Stars](https://img.shields.io/github/stars/phakhawatchu/libthai-idcard?style=social)](https://github.com/phakhawatchu/libthai-idcard)

---

**libthai-idcard** คือไลบรารีภาษา Rust สำหรับอ่านข้อมูลจากบัตรประจำตัวประชาชนไทยแบบสมาร์ทการ์ด (Thai National ID Smart Card) ผ่านเครื่องอ่านบัตรที่รองรับ PC/SC รองรับการทำงานบนหลายแพลตฟอร์ม (macOS, Linux, Windows) จัดการการสื่อสารระดับต่ำด้วยคำสั่ง APDU, การถอดรหัสข้อความภาษาไทย TIS-620 (Windows-874), การแปลงปี พ.ศ. เป็น ค.ศ. และมี FFI รองรับการเรียกใช้งานจากหลายภาษา

## 📋 สารบัญ

- [libthai-idcard](#libthai-idcard)
  - [📋 สารบัญ](#-สารบัญ)
  - [✨ ความสามารถ](#-ความสามารถ)
  - [🎯 ทำไมต้อง libthai-idcard?](#-ทำไมต้อง-libthai-idcard)
  - [ข้อกำหนด](#ข้อกำหนด)
  - [การติดตั้ง](#การติดตั้ง)
  - [การใช้งาน](#การใช้งาน)
    - [Rust](#rust)
    - [C](#c)
    - [C++](#c-1)
    - [Go](#go)
    - [Java](#java)
    - [Kotlin](#kotlin)
    - [JavaScript](#javascript)
    - [Python](#python)
    - [Ruby](#ruby)
  - [การคอมไพล์](#การคอมไพล์)
    - [Pre-built Binaries](#pre-built-binaries)
    - [Cross-compilation](#cross-compilation)
  - [โครงสร้างข้อมูล](#โครงสร้างข้อมูล)
  - [API Reference](#api-reference)
    - [ชนิดข้อมูลหลัก](#ชนิดข้อมูลหลัก)
    - [ฟังก์ชัน FFI](#ฟังก์ชัน-ffi)
  - [โครงสร้างโปรเจกต์](#โครงสร้างโปรเจกต์)
  - [คำถามที่พบบ่อย (FAQ)](#คำถามที่พบบ่อย-faq)
    - [บัตรประจำตัวประชาชนแบบ Smart Card คืออะไร?](#บัตรประจำตัวประชาชนแบบ-smart-card-คืออะไร)
    - [สามารถใช้ไลบรารีนี้โดยไม่มีเครื่องอ่านบัตรได้ไหม?](#สามารถใช้ไลบรารีนี้โดยไม่มีเครื่องอ่านบัตรได้ไหม)
    - [ใช้ภาษาโปรแกรมอะไรได้บ้าง?](#ใช้ภาษาโปรแกรมอะไรได้บ้าง)
    - [รองรับข้อมูล สปสช. (ประกันสุขภาพ) ไหม?](#รองรับข้อมูล-สปสช-ประกันสุขภาพ-ไหม)
    - [การแปลงวันที่ทำงานอย่างไร?](#การแปลงวันที่ทำงานอย่างไร)
  - [การมีส่วนร่วม](#การมีส่วนร่วม)
  - [อ้างอิง](#อ้างอิง)
  - [สัญญาอนุญาต](#สัญญาอนุญาต)

---

## ✨ ความสามารถ

| หมวดหมู่             | ความสามารถ                                                                                |
| ------------------ | ----------------------------------------------------------------------------------------- |
| **ข้อมูลบุคคล**       | อ่านเลขประจำตัวประชาชน 13 หลัก, ชื่อ (ไทย/อังกฤษ), วันเกิด, เพศ                                    |
| **ที่อยู่**            | อ่านที่อยู่ตามทะเบียนบ้าน แยกเป็นส่วนประกอบ                                                        |
| **ข้อมูลบัตร**        | อ่านผู้ออกบัตร, วันออกบัตร, วันหมดอายุ                                                            |
| **รูปถ่าย**          | อ่านรูปถ่ายใบหน้าแบบ JPEG (คืนค่าเป็น base64)                                                    |
| **เลขหลังบัตร**      | อ่านหมายเลขหลังบัตรแบบ Laser Engraved                                                        |
| **NHSO / สปสช.**   | อ่านข้อมูลสิทธิการรักษาพยาบาล: โรงพยาบาลหลัก/รอง, วันที่คุ้มครอง, ประเภทการจ่าย, จำนวนครั้งเปลี่ยนโรงพยาบาล |
| **การเข้ารหัส**      | ถอดรหัส TIS-620 (Windows-874 / ISO 8859-11) อัตโนมัติ                                         |
| **ปฏิทิน**           | แปลงปีพุทธศักราช (พ.ศ.) → คริสต์ศักราช (ค.ศ.) อัตโนมัติ                                            |
| **ตรวจจับเครื่องอ่าน** | ตรวจจับเครื่องอ่านบัตรอัตโนมัติ หรือระบุชื่อเครื่องที่ต้องการ                                              |
| **โหมด Daemon**    | มอนิเตอร์การเสียบบัตรแบบเรียลไทม์                                                               |
| **หลายภาษา**       | Rust API + ตัวอย่าง FFI ใน 8 ภาษา                                                           |

## 🎯 ทำไมต้อง libthai-idcard?

- **ไลบรารี Rust ที่พร้อมใช้งานจริง** สำหรับอ่านบัตรประชาชนไทย — ผ่านการทดสอบในแอปพลิเคชันจริง
- **ทดสอบบน 8 ภาษาผ่าน FFI** — ใช้ไลบรารีเดียวกันจาก Rust, C, C++, Go, Java, Kotlin, JavaScript, Python, Ruby
- **ข้ามแพลตฟอร์ม** — ใช้งานได้บน macOS (Intel & Apple Silicon), Linux (x86_64 & ARM64), Windows (x86_64 & ARM64)
- **ครบทุกข้อมูล** — อ่านทุกฟิลด์จากบัตร รวมถึงข้อมูล สปสช. และรูปถ่ายใบหน้า
- **การเข้ารหัสที่ถูกต้อง** — ถอดรหัส TIS-620 และแปลงวันที่ พ.ศ. เป็น ค.ศ. อย่างถูกต้อง
- **API เรียบร้อย** — โค้ด Rust ที่มี doc พร้อมตัวเลือกแบบ Builder Pattern
- **มี Binary** — ดาวน์โหลดจาก GitHub Releases ไม่ต้องคอมไพล์เอง
- **โอเพนซอร์ส** — สัญญาอนุญาต MIT / Apache 2.0

## ข้อกำหนด

- **ฮาร์ดแวร์:** เครื่องอ่านบัตร PC/SC + บัตรประจำตัวประชาชนแบบ Smart Card
- **ซอฟต์แวร์:** PC/SC Lite (`pcsclite`)
  - **macOS:** Built-in (`PCSC.framework`)
  - **Linux:** `sudo apt install libpcsclite-dev` (Debian/Ubuntu) หรือ `sudo dnf install pcsc-lite-devel` (Fedora)
  - **Windows:** Built-in (Winscard)

## การติดตั้ง

เพิ่มใน `Cargo.toml`:

```toml
[dependencies]
libthai-idcard = "0.2"
```

สำหรับภาษาอื่น ดาวน์โหลด [shared library สำเร็จรูป](https://github.com/phakhawatchu/libthai-idcard/releases/latest) สำหรับแพลตฟอร์มของคุณ แล้วทำตาม[ตัวอย่างการใช้งาน](#การใช้งาน)

## การใช้งาน

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

หรือระบุเครื่องอ่านและเปิดอ่านข้อมูลเพิ่มเติม:

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

ดูตัวอย่างเต็มได้ที่ [`examples/rust_usage.rs`](examples/rust_usage.rs)

### C

ตัวอย่างการใช้งานภาษา C อยู่ที่ [`examples/c_usage.c`](examples/c_usage.c)
สาธิตทั้งการโหลดแบบ dynamic (`dlopen`/`dlsym`) และการ link ขณะคอมไพล์

```bash
# Dynamic loading (ไม่ต้องใช้ linker flags)
cc -o c_usage examples/c_usage.c -ldl
./c_usage

# Compile-time linking
cc -o c_usage examples/c_usage.c -Ltarget/debug -lthaiidcard \
   -lpcsclite -Wl,-rpath,target/debug
./c_usage
```

### C++

ตัวอย่างการใช้งานภาษา C++ อยู่ที่
[`examples/cpp_usage.cpp`](examples/cpp_usage.cpp)
ใช้ RAII wrappers และฟีเจอร์ C++17 เพื่อโหลด shared library
และอ่านข้อมูลบัตร

```bash
# macOS / Linux:
g++ -std=c++17 -o cpp_usage examples/cpp_usage.cpp -ldl
./cpp_usage

# Windows (MinGW):
g++ -std=c++17 -o cpp_usage.exe examples/cpp_usage.cpp
./cpp_usage
```

### Go

ตัวอย่างการใช้งานภาษา Go อยู่ที่
[`examples/go_usage.go`](examples/go_usage.go)
ใช้ `cgo` เพื่อโหลด shared library ผ่าน `dlopen` เพื่ออ่านข้อมูลบัตร

```bash
go run examples/go_usage.go
```

### Java

ตัวอย่างการใช้งานภาษา Java อยู่ที่
[`examples/java_usage.java`](examples/java_usage.java)
ใช้ **JNA** (Java Native Access) ในการเรียกฟังก์ชันจาก shared library

```bash
# ใช้ jbang (ดาวน์โหลด JNA อัตโนมัติ):
jbang examples/java_usage.java

# หรือคอมไพล์และรันด้วยตนเอง (ต้องดาวน์โหลด jna.jar ก่อน):
javac -cp jna.jar examples/java_usage.java
java -cp .:jna.jar java_usage
```

### Kotlin

ตัวอย่างการใช้งานภาษา Kotlin อยู่ที่
[`examples/kotlin_usage.kt`](examples/kotlin_usage.kt)
ใช้ **JNA** (Java Native Access) ในการเรียกฟังก์ชันจาก shared library

```bash
# ใช้ jbang (ดาวน์โหลด JNA อัตโนมัติ):
jbang examples/kotlin_usage.kt

# หรือคอมไพล์และรันด้วยตนเอง (ต้องดาวน์โหลด jna.jar ก่อน):
kotlinc -cp jna.jar examples/kotlin_usage.kt
kotlin -cp .:jna.jar kotlin_usageKt
```

### JavaScript

ตัวอย่างการใช้งานภาษา JavaScript อยู่ที่
[`examples/js_usage.js`](examples/js_usage.js)
ใช้ **koffi** (ไลบรารี FFI ที่ทันสมัยสำหรับ Node.js) ในการเรียกฟังก์ชัน
จาก shared library

```bash
npm install koffi
node examples/js_usage.js
```

### Python

ตัวอย่างการใช้งานภาษา Python อยู่ที่
[`examples/python_usage.py`](examples/python_usage.py)
ใช้ `ctypes` เพื่อโหลด shared library และอ่านข้อมูลบัตร

```bash
python3 examples/python_usage.py
```

### Ruby

ตัวอย่างการใช้งานภาษา Ruby อยู่ที่
[`examples/ruby_usage.rb`](examples/ruby_usage.rb)
ใช้ `fiddle` (Ruby FFI library) เพื่อโหลด shared library และอ่านข้อมูลบัตร

```bash
ruby examples/ruby_usage.rb
```

## การคอมไพล์

```bash
# คอมไพล์ทุก target (library + examples)
make build

# คอมไพล์เฉพาะ shared library (.dylib/.so/.dll)
make shared

# สร้างไฟล์ header สำหรับ C (ต้องติดตั้ง cbindgen)
make headers

# รันตัวอย่างภาษา Rust
make example

# รันตัวอย่างภาษา C
make c-example

# รันตัวอย่างภาษา C++
make cpp-example

# รันตัวอย่างภาษา Go
make go-example

# รันตัวอย่างภาษา Java
make java-example

# รันตัวอย่างภาษา Kotlin
make kotlin-example

# รันตัวอย่างภาษา JavaScript
make js-example

# รันตัวอย่างภาษา Python
make python-example

# รันตัวอย่างภาษา Ruby
make ruby-example
```

### Pre-built Binaries

ดาวน์โหลด shared library สำเร็จรูปได้ที่
[GitHub Releases](https://github.com/phakhawatchu/libthai-idcard/releases/latest)

| แพลตฟอร์ม | สถาปัตยกรรม            | ไฟล์                                |
| -------- | --------------------- | ---------------------------------- |
| Linux    | x86_64                | `libthaiidcard-linux-x86_64.so`    |
| Linux    | ARM64 / AArch64       | `libthaiidcard-linux-arm64.so`     |
| macOS    | x86_64 (Intel)        | `libthaiidcard-macos-x86_64.dylib` |
| macOS    | ARM64 (Apple Silicon) | `libthaiidcard-macos-arm64.dylib`  |
| Windows  | x86_64                | `thaiidcard-windows-x86_64.dll`    |
| Windows  | ARM64                 | `thaiidcard-windows-arm64.dll`     |

### Cross-compilation

การคอมไพล์แบบ cross-compilation ทั้งหมดใช้ Docker
(ยกเว้น `build-win-native*` ที่ต้องติดตั้ง mingw-w64 ในเครื่อง)
การคอมไพล์สำหรับ macOS ใช้ **osxcross** เพื่อจำลอง Apple SDK
และ toolchain ภายใน Docker ที่รันบน Linux

```bash
# สร้าง Linux .so ผ่าน Docker (x86_64)
make build-linux

# สร้าง Linux .so ผ่าน Docker (ARM64)
make build-linux-arm64

# สร้าง macOS .dylib ผ่าน Docker ด้วย osxcross (Intel)
make build-mac-x64

# สร้าง macOS .dylib ผ่าน Docker ด้วย osxcross (Apple Silicon)
make build-mac

# สร้าง Windows DLL ผ่าน Docker (x86_64)
make build-win

# สร้าง Windows DLL บนเครื่องจริง (x86_64, ต้องมี mingw-w64)
make build-win-native

# สร้าง Windows DLL บนเครื่องจริง โหมด release
make build-win-native-release
```

> **หมายเหตุ:** Windows ARM64 และ Linux ARM64 มีให้ดาวน์โหลดเป็น
> [binary สำเร็จรูป](#pre-built-binaries) จาก GitHub Releases

หรือใช้ Cargo โดยตรง:

```bash
cargo build --release
cargo build --lib # shared library อย่างเดียว
```

## โครงสร้างข้อมูล

```
CardData
├── personal: Personal
│   ├── cid              — เลขประจำตัวประชาชน 13 หลัก
│   ├── name             — ชื่อเต็มภาษาไทย (คำนำหน้า, ชื่อต้น, ชื่อกลาง, นามสกุล)
│   ├── name_en          — ชื่อเต็มภาษาอังกฤษ
│   ├── dob              — วันเกิด (YYYY-MM-DD)
│   ├── gender           — M หรือ F
│   ├── card_issuer      — ผู้ออกบัตร
│   ├── issue_date       — วันออกบัตร (YYYY-MM-DD)
│   ├── expire_date      — วันบัตรหมดอายุ (YYYY-MM-DD)
│   ├── address          — ที่อยู่ตามทะเบียนบ้าน (แยกเป็นส่วนประกอบ)
│   └── face_image       — รูปถ่ายใบหน้าเป็น base64 JPEG
├── card: Card
│   └── laser_id         — หมายเลขหลังบัตร
└── nhso: Nhso
      ├── main_inscl     — สิทธิการรักษาหลัก
      ├── sub_inscl      — สิทธิการรักษารอง
      ├── main_hospital  — โรงพยาบาลหลัก
      ├── sub_hospital   — โรงพยาบาลรอง
      ├── paid_type      — ประเภทการจ่าย
      ├── issue_date     — วันที่เริ่มคุ้มครอง (สปสช.)
      ├── expire_date    — วันที่สิ้นสุดคุ้มครอง (สปสช.)
      ├── update_date    — วันที่อัปเดตล่าสุด
      └── change_hospital_amount — จำนวนครั้งที่เปลี่ยนโรงพยาบาล
```

## API Reference

เอกสาร API ฉบับเต็มอยู่ที่ [docs.rs/libthai-idcard](https://docs.rs/libthai-idcard/latest/thaiidcard/)

### ชนิดข้อมูลหลัก

- **`SmartCard`** — ตัวหลักสำหรับอ่านบัตรประจำตัวประชาชน
  - `SmartCard::new()` — สร้างอินสแตนซ์ใหม่
  - `SmartCard::list_readers()` — แสดงรายชื่อเครื่องอ่าน PC/SC ทั้งหมด
  - `card.read(reader_name, opts)` — อ่านบัตรครั้งเดียว
  - `card.start_daemon(opts)` — มอนิเตอร์เครื่องอ่านแบบต่อเนื่อง
- **`Options`** — ตั้งค่าส่วนข้อมูลที่ต้องการอ่าน
  - `show_nhso_data` — อ่านข้อมูล สปสช.
  - `show_laser_data` — อ่านหมายเลขหลังบัตร
  - `show_face_image` — อ่านรูปถ่ายใบหน้า (base64 JPEG)
- **`CardData`** — ข้อมูลทั้งหมดจากบัตร
- **`Personal`** — ข้อมูลประจำตัวบุคคล
- **`Nhso`** — ข้อมูลสิทธิการรักษาพยาบาล
- **`Card`** — ข้อมูลบัตร (หมายเลขหลังบัตร)

### ฟังก์ชัน FFI

| ฟังก์ชัน                                             | วัตถุประสงค์                   |
| ------------------------------------------------- | --------------------------- |
| `thaiidcard_read_card(reader, json_opts) → char*` | อ่านบัตร คืนค่า JSON            |
| `thaiidcard_list_readers() → char*`               | รายชื่อเครื่องอ่าน คืนค่า JSON     |
| `thaiidcard_free_string(ptr)`                     | คืนหน่วยความจำสตริงที่ได้จากไลบรารี |

## โครงสร้างโปรเจกต์

```
├── Cargo.toml
├── Makefile
├── Dockerfile.build       — Docker สำหรับ cross-compilation (osxcross)
├── src/
│   ├── lib.rs             — API หลัก (SmartCard, start_daemon)
│   ├── ffi.rs             — FFI สำหรับภาษา C
│   ├── model.rs           — ชนิดข้อมูลและ parsing helpers
│   ├── apdu.rs            — ค่าคงที่สำหรับคำสั่ง APDU
│   ├── reader.rs          — Low-level PC/SC operations
│   ├── personal.rs        — อ่านข้อมูลบุคคล
│   ├── nhso.rs            — อ่านข้อมูล สปสช.
│   ├── laser.rs           — อ่านหมายเลขหลังบัตร
│   └── options.rs         — ตัวเลือกการตั้งค่า
└── examples/
    ├── rust_usage.rs      — ตัวอย่างภาษา Rust
    ├── c_usage.c          — ตัวอย่างภาษา C
    ├── cpp_usage.cpp      — ตัวอย่างภาษา C++
    ├── go_usage.go        — ตัวอย่างภาษา Go
    ├── java_usage.java    — ตัวอย่างภาษา Java
    ├── kotlin_usage.kt    — ตัวอย่างภาษา Kotlin
    ├── js_usage.js        — ตัวอย่างภาษา JavaScript / Node.js
    ├── python_usage.py    — ตัวอย่างภาษา Python
    └── ruby_usage.rb      — ตัวอย่างภาษา Ruby
```

## คำถามที่พบบ่อย (FAQ)

### บัตรประจำตัวประชาชนแบบ Smart Card คืออะไร?

บัตรประจำตัวประชาชนแบบ Smart Card (Thai National ID Smart Card) คือบัตรประจำตัวประชาชนที่มีชิพอัจฉริยะ ออกโดยกรมการปกครอง กระทรวงมหาดไทย เก็บข้อมูลประจำตัวประชาชน รูปถ่ายใบหน้าแบบ JPEG และข้อมูลสิทธิการรักษาพยาบาล (สปสช.) สามารถอ่านข้อมูลผ่านคำสั่ง APDU ผ่าน PC/SC

### สามารถใช้ไลบรารีนี้โดยไม่มีเครื่องอ่านบัตรได้ไหม?

ไม่ได้ — ต้องใช้เครื่องอ่านบัตรที่รองรับ PC/SC และบัตรประจำตัวประชาชนจริง ไลบรารีนี้สื่อสารกับบัตรผ่านฮาร์ดแวร์เท่านั้น

### ใช้ภาษาโปรแกรมอะไรได้บ้าง?

ไลบรารีเขียนด้วย Rust แต่มี C-compatible FFI ทำให้เรียกใช้จากภาษาใดก็ได้ที่รองรับ FFI มีตัวอย่าง 8 ภาษา: C, C++, Go, Java, Kotlin, JavaScript (Node.js ผ่าน koffi), Python (ผ่าน ctypes), Ruby (ผ่าน fiddle)

### รองรับข้อมูล สปสช. (ประกันสุขภาพ) ไหม?

รองรับ ตั้งค่า `Options::show_nhso_data` เป็น `true` เพื่ออ่านข้อมูลสิทธิการรักษา โรงพยาบาลหลัก/รอง วันที่คุ้มครอง และอื่น ๆ

### การแปลงวันที่ทำงานอย่างไร?

บัตรประจำตัวประชาชนไทยเก็บวันที่ในรูปแบบพุทธศักราช (พ.ศ.) ไลบรารีจะแปลงเป็นคริสต์ศักราช (ค.ศ.) โดยอัตโนมัติ วันที่ที่ได้จะเป็นรูปแบบ `YYYY-MM-DD`

## การมีส่วนร่วม

โปรดดู [CONTRIBUTING.md](CONTRIBUTING.md) สำหรับแนวทาง

เป้าหมายการพัฒนา:
- เพิ่มภาษา binding เพิ่มเติม (Swift, C#, WASM)
- เพิ่ม test coverage ด้วย card simulator
- ปรับปรุง doc ให้ดีขึ้น

## อ้างอิง

- [Thai National ID Card APDU Specification](https://github.com/chakphanu/ThaiNationalIDCard/blob/master/APDU.md)
- [go-thai-smartcard NHSO APDU Implementation](https://github.com/somprasongd/go-thai-smartcard/blob/main/pkg/apdu/nhso.go)

## สัญญาอนุญาต

เผยแพร่ภายใต้สัญญาอนุญาตใดในสองฉบับต่อไปนี้:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

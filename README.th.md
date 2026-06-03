> **🌐 ภาษา:** [English](README.md) · ไทย

# libthai-idcard

[![Crates.io](https://img.shields.io/crates/v/libthai-idcard)](https://crates.io/crates/libthai-idcard)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](https://github.com/phakhawatchu/libthai-idcard)
[![CI](https://github.com/phakhawatchu/libthai-idcard/actions/workflows/ci.yml/badge.svg)](https://github.com/phakhawatchu/libthai-idcard/actions/workflows/ci.yml)
[![MSRV](https://img.shields.io/badge/rustc-1.81%2B-lightgrey)](https://github.com/phakhawatchu/libthai-idcard)

ไลบรารีภาษา Rust สำหรับอ่านข้อมูลจาก **บัตรประจำตัวประชาชนแบบ Smart Card**
ผ่าน PC/SC พร้อมตัวอย่างการใช้งานในภาษา **C**, **Go**, **Python** และ **Ruby**

## ภาพรวม

ไลบรารีนี้สามารถอ่านบัตรประจำตัวประชาชนแบบ Smart Card ที่เก็บข้อมูลประจำตัวประชาชน
รูปถ่ายใบหน้า ข้อมูลประกันสุขภาพ (NHSO) และเลขหลังบัตร ผ่านการการสื่อสารแบบ low-level APDU 
ถอดรหัสข้อความภาษาไทยแบบ แปลงปี พ.ศ. เป็น ค.ศ. และสามารถใช้งานได้จากหลายภาษา

### ความสามารถ

- ✅ อ่านเลขประจำตัวประชาชน ชื่อ (ไทย/อังกฤษ) วันเดือนปีเกิด เพศ
- ✅ อ่านที่อยู่ตามทะเบียนบ้าน (แยกเป็นส่วนประกอบ)
- ✅ อ่านผู้ออกบัตร วันออกบัตร วันบัตรหมดอายุ
- ✅ อ่านรูปถ่ายใบหน้าแบบ JPEG (คืนค่าเป็น base64)
- ✅ อ่านหมายเลขหลังบัตร
- ✅ อ่านข้อมูลสิทธิการรักษาพยาบาล (โรงพยาบาลหลัก/รอง, วันที่คุ้มครอง, ฯลฯ)
- ✅ แปลงปีพุทธศักราช → คริสต์ศักราช
- ✅ ถอดรหัสข้อความภาษาไทย TIS-620 (Windows-874)
- ✅ ตรวจจับเครื่องอ่านบัตรอัตโนมัติ หรือระบุชื่อเครื่องที่ต้องการ
- ✅ โหมด Daemon สำหรับมอนิเตอร์การเสียบบัตร
- ✅ ตัวอย่างการใช้งานภาษา C (โหลดแบบ dynamic หรือ link-time)
- ✅ ตัวอย่างการใช้งานภาษา Go (ผ่าน `cgo`)
- ✅ ตัวอย่างการใช้งานภาษา Python (ผ่าน `ctypes`)
- ✅ ตัวอย่างการใช้งานภาษา Ruby (ผ่าน `fiddle`)


## ข้อกำหนด

- **ฮาร์ดแวร์:** เครื่องอ่านบัตรที่รองรับ PC/SC และบัตรประจำตัวประชาชนแบบ Smart Card
- **ซอฟต์แวร์:** PC/SC Lite (`pcsclite`) — มีติดตั้งอยู่แล้วบน macOS และ Linux เป็นส่วนใหญ่
  - **macOS:** Built-in (`PCSC.framework`)
  - **Linux:** `sudo apt install libpcsclite-dev` (Debian/Ubuntu) หรือ `sudo dnf install pcsc-lite-devel` (Fedora)
  - **Windows:** Built-in (Winscard)

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

### Go

ตัวอย่างการใช้งานภาษา Go อยู่ที่
[`examples/go_usage.go`](examples/go_usage.go)
ใช้ `cgo` เพื่อโหลด shared library ผ่าน `dlopen` เพื่ออ่านข้อมูลบัตร

```bash
go run examples/go_usage.go
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

# คอมไพล์เฉพาะ shared library (.dylib/.so)
make shared

# สร้างไฟล์ header สำหรับ C (ต้องติดตั้ง cbindgen)
make headers

# รันตัวอย่างภาษา Rust
make example

# รันตัวอย่างภาษา C
make c-example

# รันตัวอย่างภาษา Go
make go-example

# รันตัวอย่างภาษา Python
make python-example

# รันตัวอย่างภาษา Ruby
make ruby-example
```

### Cross-compilation

การคอมไพล์แบบ cross-compilation ทั้งหมดใช้ Docker
(ยกเว้น `build-win-native*` ที่ต้องติดตั้ง mingw-w64 ในเครื่อง)
การคอมไพล์สำหรับ macOS ใช้ **osxcross** เพื่อจำลอง Apple SDK
และ toolchain ภายใน Docker ที่รันบน Linux

```bash
# สร้าง Linux .so ผ่าน Docker
make build-linux

# สร้าง macOS .dylib ผ่าน Docker ด้วย osxcross (Apple Silicon)
make build-mac

# สร้าง macOS .dylib ผ่าน Docker ด้วย osxcross (Intel)
make build-mac-x64

# สร้าง Windows DLL ผ่าน Docker
make build-win

# สร้าง Windows DLL บนเครื่องจริง (ต้องมี mingw-w64)
make build-win-native

# สร้าง Windows DLL บนเครื่องจริง โหมด release
make build-win-native-release
```

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
    ├── go_usage.go        — ตัวอย่างภาษา Go
    ├── python_usage.py    — ตัวอย่างภาษา Python
    └── ruby_usage.rb      — ตัวอย่างภาษา Ruby
```

## อ้างอิง

- [Thai National ID Card APDU Specification](https://github.com/chakphanu/ThaiNationalIDCard/blob/master/APDU.md)
- [go-thai-smartcard NHSO APDU Implementation](https://github.com/somprasongd/go-thai-smartcard/blob/main/pkg/apdu/nhso.go)

## สัญญาอนุญาต

เผยแพร่ภายใต้สัญญาอนุญาตใดในสองฉบับต่อไปนี้:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

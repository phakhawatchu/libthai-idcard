//! One-shot Thai smart card read example.
//!
//! This example links to the library at compile time (not dynamic loading).
//! It works on macOS, Linux, and Windows via:
//!     cargo run --example rust_usage [reader_name]
//!
//! For the C FFI / shared-library approach, see the c_usage.c, go_usage.go,
//! python_usage.py, and ruby_usage.rb examples.

use std::env;
use thaiidcard::{model, Options, SmartCard};

fn main() {
    let reader_name = env::args().nth(1);

    match SmartCard::list_readers() {
        Ok(readers) => {
            println!("Found {} reader(s):", readers.len());
            for (i, r) in readers.iter().enumerate() {
                println!("  [{}] {}", i, r);
            }
        }
        Err(e) => eprintln!("Failed to list readers: {}", e),
    }

    let opts = Options {
        show_nhso_data: true,
        ..Options::default()
    };

    let card = SmartCard::new();
    println!("\nWaiting for card...");
    match card.read(reader_name.as_deref(), &opts) {
        Ok(data) => {
            print_card_data(&data);
        }
        Err(e) => eprintln!("Read error: {}", e),
    }
}

fn print_card_data(data: &model::CardData) {
    if let Some(p) = &data.personal {
        println!("\n=== Personal Information ===");
        println!("  CID:               {}", p.cid);
        println!("  Name (TH):         {}", p.name.full_name);
        println!("  Name (EN):         {}", p.name_en.full_name);
        println!("  DOB:               {}", p.dob);
        println!("  Gender:            {}", p.gender);
        println!("  Card Issuer:       {}", p.card_issuer);
        println!("  Issue Date:        {}", p.issue_date);
        println!("  Expire Date:       {}", p.expire_date);
        println!("  Address:           {}", p.address.full_address);
        if !p.face_image.is_empty() {
            println!("  Face Image:        [{} bytes base64]", p.face_image.len());
        }
    }

    if let Some(c) = &data.card {
        if !c.laser_id.is_empty() {
            println!("\n=== Card Info ===");
            println!("  Laser ID:          {}", c.laser_id);
        }
    }

    if let Some(n) = &data.nhso {
        if !n.main_inscl.is_empty() {
            println!("\n=== NHSO Information ===");
            println!("  Main Inscl:        {}", n.main_inscl);
            println!("  Sub Inscl:         {}", n.sub_inscl);
            println!("  Main Hosp:         {}", n.main_hospital);
            println!("  Sub Hosp:          {}", n.sub_hospital);
            println!("  Paid Type:         {}", n.paid_type);
            println!("  NHSO Issue:        {}", n.issue_date);
            println!("  NHSO Expire:       {}", n.expire_date);
            println!("  NHSO Update:       {}", n.update_date);
            println!("  Change Hosp:       {}", n.change_hospital_amount);
        }
    }
}

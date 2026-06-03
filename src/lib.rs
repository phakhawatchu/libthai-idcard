//! `libthai-idcard` — A Rust library for reading Thai National ID smart cards
//! via PC/SC card readers. Supports personal information, NHSO insurance data,
//! and the laser-engraved card serial number.
//!
//! # Basic usage
//!
//! ```no_run
//! use thaiidcard::{SmartCard, Options};
//!
//! let card = SmartCard::new();
//! let data = card.read(None, &Options::default()).unwrap();
//! println!("Name: {}", data.personal.unwrap().name.full_name);
//! ```
#![allow(clippy::large_enum_variant)]

pub mod apdu;
pub mod ffi;
pub mod model;
pub mod options;
pub mod reader;

mod laser;
mod nhso;
mod personal;

pub use options::Options;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use pcsc::*;

use crate::reader::CardError;

/// Events emitted by [`SmartCard::start_daemon`].
#[derive(Debug)]
pub enum Event {
    CardInserted { reader: String },
    CardData(model::CardData),
    CardRemoved { reader: String },
    Error(String),
}

/// Main handle for reading Thai National ID smart cards.
pub struct SmartCard;

impl SmartCard {
    /// Create a new `SmartCard` instance.
    pub fn new() -> Self {
        Self
    }

    /// Return all available PC/SC smart card reader names.
    pub fn list_readers() -> Result<Vec<String>, CardError> {
        let ctx = reader::establish_context()?;
        reader::list_readers(&ctx)
    }

    /// Perform a single card read. If `reader_name` is `None`, the first
    /// available reader with a card present is used.
    pub fn read(
        &self,
        reader_name: Option<&str>,
        opts: &Options,
    ) -> Result<model::CardData, CardError> {
        let ctx = reader::establish_context()?;

        let readers = match reader_name {
            Some(name) => vec![name.to_string()],
            None => reader::list_readers(&ctx)?,
        };

        let idx = reader::wait_for_card(&ctx, &readers)?;
        let reader = &readers[idx];

        self.read_card(&ctx, reader, opts)
    }

    /// Monitor readers continuously and emit events on the returned channel.
    /// The daemon runs until the receiver is dropped.
    pub fn start_daemon(opts: Options) -> mpsc::Receiver<Event> {
        let (tx, rx) = mpsc::channel();
        let card = Self;

        thread::spawn(move || loop {
            let ctx = match reader::establish_context() {
                Ok(c) => c,
                Err(e) => {
                    let _ = tx.send(Event::Error(e.to_string()));
                    thread::sleep(Duration::from_secs(2));
                    continue;
                }
            };

            let readers = match reader::list_readers(&ctx) {
                Ok(r) => r,
                Err(_) => {
                    thread::sleep(Duration::from_secs(2));
                    continue;
                }
            };

            let idx = match reader::wait_for_card(&ctx, &readers) {
                Ok(i) => i,
                Err(e) => {
                    let _ = tx.send(Event::Error(e.to_string()));
                    continue;
                }
            };

            let reader_name = readers[idx].clone();
            let _ = tx.send(Event::CardInserted {
                reader: reader_name.clone(),
            });

            match card.read_card(&ctx, &reader_name, &opts) {
                Ok(data) => {
                    let _ = tx.send(Event::CardData(data));
                }
                Err(e) => {
                    let _ = tx.send(Event::Error(e.to_string()));
                }
            }

            let _ = reader::wait_for_card_removal(&ctx, idx, &readers);
            let _ = tx.send(Event::CardRemoved {
                reader: reader_name,
            });
        });

        rx
    }

    fn read_card(
        &self,
        ctx: &Context,
        reader: &str,
        opts: &Options,
    ) -> Result<model::CardData, CardError> {
        let card = reader::connect_card(ctx, reader)?;

        let status = card
            .status2_owned()
            .map_err(|e| CardError::Context(e.to_string()))?;
        let resp_cmd = reader::get_response_command(status.atr());

        let personal = personal::read_personal(&card, &resp_cmd, opts.show_face_image);

        let card_info = if opts.show_laser_data {
            Some(laser::read_laser_id(&card, &resp_cmd))
        } else {
            None
        };

        let nhso_data = if opts.show_nhso_data {
            Some(nhso::read_nhso(&card, &resp_cmd))
        } else {
            None
        };

        drop(card);

        Ok(model::CardData {
            personal: Some(personal),
            card: card_info,
            nhso: nhso_data,
        })
    }
}

impl Default for SmartCard {
    fn default() -> Self {
        Self::new()
    }
}

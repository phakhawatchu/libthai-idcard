//! Laser ID (card serial number) reader.

use pcsc::Card;

use crate::apdu;
use crate::model;
use crate::reader::{self, transmit_read_laser_id};

/// Read the laser-engraved card serial number.
pub fn read_laser_id(card: &Card, resp_cmd: &[u8]) -> model::Card {
    let _ = reader::transmit_select(card, apdu::card::SELECT);

    model::Card {
        laser_id: transmit_read_laser_id(card, apdu::card::LASER_ID, resp_cmd).unwrap_or_default(),
    }
}

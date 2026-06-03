//! NHSO (National Health Security Office) insurance data reader.

use pcsc::Card;

use crate::apdu;
use crate::model;
use crate::reader::{self, transmit_read};

/// Read NHSO insurance data from the card.
pub fn read_nhso(card: &Card, resp_cmd: &[u8]) -> model::Nhso {
    let _ = reader::transmit_select(card, apdu::nhso::SELECT);

    model::Nhso {
        main_inscl: read_field(card, resp_cmd, apdu::nhso::MAIN_INSCL, true),
        sub_inscl: read_field(card, resp_cmd, apdu::nhso::SUB_INSCL, true),
        main_hospital: read_field(card, resp_cmd, apdu::nhso::MAIN_HOSPITAL_NAME, true),
        sub_hospital: read_field(card, resp_cmd, apdu::nhso::SUB_HOSPITAL_NAME, true),
        paid_type: read_field(card, resp_cmd, apdu::nhso::PAID_TYPE, true),
        issue_date: read_date(card, resp_cmd, apdu::nhso::ISSUE_DATE),
        expire_date: read_date(card, resp_cmd, apdu::nhso::EXPIRE_DATE),
        update_date: read_date(card, resp_cmd, apdu::nhso::UPDATE_DATE),
        change_hospital_amount: read_field(
            card,
            resp_cmd,
            apdu::nhso::CHANGE_HOSPITAL_AMOUNT,
            false,
        ),
    }
}

fn read_field(card: &Card, resp: &[u8], cmd: &[u8], tis620: bool) -> String {
    transmit_read(card, cmd, resp, tis620).unwrap_or_default()
}

fn read_date(card: &Card, resp: &[u8], cmd: &[u8]) -> String {
    transmit_read(card, cmd, resp, false)
        .map(|s| model::format_date(&s))
        .unwrap_or_default()
}

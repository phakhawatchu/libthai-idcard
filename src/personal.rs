//! Personal identification data reader.

use pcsc::Card;

use crate::apdu;
use crate::model::{self, parse_name};
use crate::reader::{self, transmit_read, transmit_read_bytes};

/// Read personal identification data from the card.
pub fn read_personal(card: &Card, resp_cmd: &[u8], with_face: bool) -> model::Personal {
    let _ = reader::transmit_select(card, apdu::personal::SELECT);

    model::Personal {
        cid: read_cid(card, resp_cmd),
        name: parse_name(&read_name_thai(card, resp_cmd)),
        name_en: parse_name(&read_name_en(card, resp_cmd)),
        dob: read_dob(card, resp_cmd),
        gender: read_gender(card, resp_cmd),
        card_issuer: read_card_issuer(card, resp_cmd),
        issue_date: read_issue_date(card, resp_cmd),
        expire_date: read_expire_date(card, resp_cmd),
        address: model::parse_address(&read_address(card, resp_cmd)),
        face_image: if with_face {
            read_face_img(card, resp_cmd)
        } else {
            String::new()
        },
    }
}

fn read_cid(card: &Card, resp: &[u8]) -> String {
    transmit_read(card, apdu::personal::CID, resp, false).unwrap_or_default()
}

fn read_name_thai(card: &Card, resp: &[u8]) -> String {
    transmit_read(card, apdu::personal::NAME_THAI, resp, true).unwrap_or_default()
}

fn read_name_en(card: &Card, resp: &[u8]) -> String {
    transmit_read(card, apdu::personal::NAME_EN, resp, true).unwrap_or_default()
}

fn read_dob(card: &Card, resp: &[u8]) -> String {
    transmit_read(card, apdu::personal::DOB, resp, false)
        .map(|s| model::format_date(&s))
        .unwrap_or_default()
}

fn read_gender(card: &Card, resp: &[u8]) -> String {
    transmit_read(card, apdu::personal::GENDER, resp, false).unwrap_or_default()
}

fn read_card_issuer(card: &Card, resp: &[u8]) -> String {
    transmit_read(card, apdu::personal::CARD_ISSUER, resp, true).unwrap_or_default()
}

fn read_issue_date(card: &Card, resp: &[u8]) -> String {
    transmit_read(card, apdu::personal::ISSUE_DATE, resp, false)
        .map(|s| model::format_date(&s))
        .unwrap_or_default()
}

fn read_expire_date(card: &Card, resp: &[u8]) -> String {
    transmit_read(card, apdu::personal::EXPIRE_DATE, resp, false)
        .map(|s| model::format_date(&s))
        .unwrap_or_default()
}

fn read_address(card: &Card, resp: &[u8]) -> String {
    transmit_read(card, apdu::personal::ADDRESS, resp, true).unwrap_or_default()
}

fn read_face_img(card: &Card, resp: &[u8]) -> String {
    let mut raw_data = Vec::new();
    for cmd in apdu::personal::FACE_IMAGE {
        let chunk = transmit_read_bytes(card, cmd, resp).unwrap_or_default();
        if chunk.is_empty() {
            break;
        }
        raw_data.extend_from_slice(&chunk);
    }

    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(&raw_data)
}

//! Low-level PC/SC card operations and APDU transport.

use pcsc::*;
use std::error::Error;
use std::ffi::CString;
use std::fmt;
use std::time::Duration;

/// Errors returned by card operations.
#[derive(Debug)]
pub enum CardError {
    NoReaders,
    CardNotFound,
    Transmit(String),
    Context(String),
    Connect(String),
}

impl fmt::Display for CardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoReaders => write!(f, "no smart card readers available"),
            Self::CardNotFound => write!(f, "no card present"),
            Self::Transmit(msg) => write!(f, "transmit: {msg}"),
            Self::Context(msg) => write!(f, "context: {msg}"),
            Self::Connect(msg) => write!(f, "connect: {msg}"),
        }
    }
}

impl Error for CardError {}

/// Establish a PC/SC context.
pub fn establish_context() -> Result<Context, CardError> {
    Context::establish(Scope::User).map_err(|e| CardError::Context(e.to_string()))
}

/// List available card reader names.
pub fn list_readers(ctx: &Context) -> Result<Vec<String>, CardError> {
    let mut buf = [0u8; 2048];
    let readers = ctx
        .list_readers(&mut buf)
        .map_err(|e| CardError::Context(e.to_string()))?;
    let result: Vec<String> = readers
        .filter_map(|r| r.to_str().ok())
        .map(|s| s.to_string())
        .collect();
    if result.is_empty() {
        return Err(CardError::NoReaders);
    }
    Ok(result)
}

/// Helper to create a CString from a string slice for pcsc APIs.
fn to_c_str(s: &str) -> CString {
    CString::new(s).unwrap_or_else(|_| CString::new("").unwrap())
}

/// Wait until a card is inserted into any of the given readers.
pub fn wait_for_card(ctx: &Context, readers: &[String]) -> Result<usize, CardError> {
    let mut states: Vec<ReaderState> = readers
        .iter()
        .map(|r| ReaderState::new(to_c_str(r), State::UNAWARE))
        .collect();

    loop {
        ctx.get_status_change(Duration::from_secs(u64::MAX), &mut states)
            .map_err(|e| CardError::Context(e.to_string()))?;

        for (i, state) in states.iter().enumerate() {
            if state.event_state().intersects(State::PRESENT) {
                return Ok(i);
            }
        }
        for state in &mut states {
            state.sync_current_state();
        }
    }
}

/// Wait until the card is removed from its reader.
pub fn wait_for_card_removal(
    ctx: &Context,
    reader_idx: usize,
    readers: &[String],
) -> Result<(), CardError> {
    let mut states = vec![ReaderState::new(
        to_c_str(&readers[reader_idx]),
        State::UNAWARE,
    )];

    loop {
        ctx.get_status_change(Duration::from_secs(u64::MAX), &mut states)
            .map_err(|e| CardError::Context(e.to_string()))?;

        if states[0].event_state().intersects(State::EMPTY) {
            return Ok(());
        }
        states[0].sync_current_state();
    }
}

/// Connect to the card in the specified reader (exclusive, any protocol).
pub fn connect_card(ctx: &Context, reader: &str) -> Result<Card, CardError> {
    let c_reader = to_c_str(reader);
    ctx.connect(c_reader.as_c_str(), ShareMode::Exclusive, Protocols::ANY)
        .map_err(|e| CardError::Connect(e.to_string()))
}

/// Get the GET RESPONSE command prefix based on the card ATR.
pub fn get_response_command(atr: &[u8]) -> Vec<u8> {
    if atr.len() >= 2 && atr[0] == 0x3B && atr[1] == 0x67 {
        vec![0x00, 0xC0, 0x00, 0x01]
    } else {
        vec![0x00, 0xC0, 0x00, 0x00]
    }
}

/// Maximum APDU buffer size (short APDU max is 264 for 255-byte payload + status).
const BUF_SIZE: usize = 264;

/// Send a READ BINARY APDU followed by GET RESPONSE.
/// If `is_tis620`, the response is decoded from Windows-874 (TIS-620) to UTF-8.
/// The trailing 2-byte status word is trimmed.
pub fn transmit_read(
    card: &Card,
    cmd: &[u8],
    get_resp_prefix: &[u8],
    is_tis620: bool,
) -> Result<String, CardError> {
    let _ = transmit_raw(card, cmd)?;

    let mut get_resp = get_resp_prefix.to_vec();
    get_resp.push(cmd[cmd.len() - 1]);
    let rsp = transmit_raw(card, &get_resp)?;

    let mut payload = strip_status(&rsp);
    if is_tis620 {
        let (decoded, _, _) = encoding_rs::WINDOWS_874.decode(&payload);
        payload = decoded.as_bytes().to_vec();
    }
    Ok(String::from_utf8_lossy(&payload).trim().to_string())
}

/// Read the laser ID using a fixed 0x10 GET RESPONSE length.
pub fn transmit_read_laser_id(
    card: &Card,
    cmd: &[u8],
    get_resp_prefix: &[u8],
) -> Result<String, CardError> {
    let _ = transmit_raw(card, cmd)?;

    let mut get_resp = get_resp_prefix.to_vec();
    get_resp.push(0x10);
    let rsp = transmit_raw(card, &get_resp)?;

    let payload: Vec<u8> = strip_status(&rsp)
        .into_iter()
        .filter(|&b| b != 0x00)
        .collect();
    Ok(String::from_utf8_lossy(&payload).trim().to_string())
}

/// Like `transmit_read` but returns raw bytes without UTF-8 conversion.
/// Used for binary data such as face images.
pub fn transmit_read_bytes(
    card: &Card,
    cmd: &[u8],
    get_resp_prefix: &[u8],
) -> Result<Vec<u8>, CardError> {
    let _ = transmit_raw(card, cmd)?;
    let mut get_resp = get_resp_prefix.to_vec();
    get_resp.push(cmd[cmd.len() - 1]);
    let rsp = transmit_raw(card, &get_resp)?;
    Ok(strip_status(&rsp))
}

/// Send a SELECT APDU to choose an applet on the card.
pub fn transmit_select(card: &Card, cmd: &[u8]) -> Result<(), CardError> {
    transmit_raw(card, cmd).map(|_| ())
}

fn transmit_raw(card: &Card, cmd: &[u8]) -> Result<Vec<u8>, CardError> {
    let mut buf = [0u8; BUF_SIZE];
    let rsp = card
        .transmit(cmd, &mut buf)
        .map_err(|e| CardError::Transmit(e.to_string()))?;
    Ok(rsp.to_vec())
}

fn strip_status(data: &[u8]) -> Vec<u8> {
    if data.len() < 2 {
        Vec::new()
    } else {
        data[..data.len() - 2].to_vec()
    }
}

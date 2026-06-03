//! C-compatible FFI for libthai-idcard.
//!
//! # Usage from C
//!
//! ```c
//! #include <stdlib.h>
//!
//! // Opaque handle
//! typedef struct thaiid_card_data thaiid_card_data;
//!
//! // Functions
//! thaiid_card_data* thaiid_read(const char* reader, int face, int nhso, int laser);
//! void              thaiid_free(thaiid_card_data* d);
//! const char*       thaiid_get_cid(const thaiid_card_data* d);
//! const char*       thaiid_get_name_thai(const thaiid_card_data* d);
//! const char*       thaiid_get_name_en(const thaiid_card_data* d);
//! const char*       thaiid_get_dob(const thaiid_card_data* d);
//! const char*       thaiid_get_gender(const thaiid_card_data* d);
//! const char*       thaiid_get_card_issuer(const thaiid_card_data* d);
//! const char*       thaiid_get_issue_date(const thaiid_card_data* d);
//! const char*       thaiid_get_expire_date(const thaiid_card_data* d);
//! const char*       thaiid_get_address(const thaiid_card_data* d);
//! const char*       thaiid_get_face_image(const thaiid_card_data* d);
//! const char*       thaiid_get_laser_id(const thaiid_card_data* d);
//! const char*       thaiid_get_main_inscl(const thaiid_card_data* d);
//! const char*       thaiid_get_main_hospital(const thaiid_card_data* d);
//! const char*       thaiid_get_last_error(void);
//! ```
//!
//! Link with: `-lthaiidcard -lpcsclite`
#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;

use crate::{model, Options, SmartCard};

static LAST_ERROR: Mutex<Option<String>> = Mutex::new(None);

fn set_error(msg: String) {
    if let Ok(mut e) = LAST_ERROR.lock() {
        *e = Some(msg);
    }
}

/// Return the last error message. The returned string is valid until the next
/// call to any thaiid_* function. Returns NULL if no error occurred.
#[no_mangle]
pub extern "C" fn thaiid_get_last_error() -> *const c_char {
    let guard = match LAST_ERROR.lock() {
        Ok(g) => g,
        _ => return std::ptr::null(),
    };
    match guard.as_ref() {
        Some(s) => s.as_ptr() as *const c_char,
        None => std::ptr::null(),
    }
}

/// Opaque handle for card data. Allocated by `thaiid_read`, freed by `thaiid_free`.
pub struct ThaiIdCardData {
    cid: CString,
    name_thai: CString,
    name_en: CString,
    dob: CString,
    gender: CString,
    card_issuer: CString,
    issue_date: CString,
    expire_date: CString,
    address: CString,
    face_image: CString,
    laser_id: CString,
    main_inscl: CString,
    sub_inscl: CString,
    main_hospital: CString,
    sub_hospital: CString,
    paid_type: CString,
    nhso_issue_date: CString,
    nhso_expire_date: CString,
    nhso_update_date: CString,
    change_hospital_amount: CString,
}

fn to_cstr(s: &str) -> CString {
    CString::new(s).unwrap_or_else(|_| CString::new("").unwrap())
}

/// Read a Thai National ID smart card.
///
/// - `reader`: reader name, or NULL for auto-detect
/// - `face`: non-zero to read face image
/// - `nhso`: non-zero to read NHSO data
/// - `laser`: non-zero to read laser ID
///
/// Returns an opaque handle, or NULL on error. Free with `thaiid_free`.
#[no_mangle]
pub extern "C" fn thaiid_read(
    reader: *const c_char,
    face: i32,
    nhso: i32,
    laser: i32,
) -> *mut ThaiIdCardData {
    let reader_name = if reader.is_null() {
        None
    } else {
        match unsafe { CStr::from_ptr(reader) }.to_str() {
            Ok(s) if !s.is_empty() => Some(s),
            _ => None,
        }
    };

    let opts = Options {
        reader_name: reader_name.map(|s| s.to_string()),
        show_face_image: face != 0,
        show_nhso_data: nhso != 0,
        show_laser_data: laser != 0,
    };

    let card = SmartCard::new();
    match card.read(reader_name, &opts) {
        Ok(data) => {
            let handle = into_handle(data);
            set_error(String::new());
            handle
        }
        Err(e) => {
            set_error(e.to_string());
            std::ptr::null_mut()
        }
    }
}

/// Free card data allocated by `thaiid_read`.
#[no_mangle]
pub extern "C" fn thaiid_free(data: *mut ThaiIdCardData) {
    if !data.is_null() {
        unsafe {
            let _ = Box::from_raw(data);
        }
    }
}

macro_rules! c_getter {
    ($name:ident, $field:ident, $doc:expr) => {
        #[doc = $doc]
        #[no_mangle]
        pub extern "C" fn $name(data: *const ThaiIdCardData) -> *const c_char {
            if data.is_null() {
                return std::ptr::null();
            }
            unsafe { (*data).$field.as_ptr() }
        }
    };
}

c_getter!(thaiid_get_cid, cid, "Citizen ID (13 digits).");
c_getter!(thaiid_get_name_thai, name_thai, "Full name in Thai.");
c_getter!(thaiid_get_name_en, name_en, "Full name in English.");
c_getter!(thaiid_get_dob, dob, "Date of birth (YYYY-MM-DD).");
c_getter!(thaiid_get_gender, gender, "Gender (M/F).");
c_getter!(
    thaiid_get_card_issuer,
    card_issuer,
    "Card issuing authority."
);
c_getter!(
    thaiid_get_issue_date,
    issue_date,
    "Card issue date (YYYY-MM-DD)."
);
c_getter!(
    thaiid_get_expire_date,
    expire_date,
    "Card expiry date (YYYY-MM-DD)."
);
c_getter!(thaiid_get_address, address, "Registered address.");
c_getter!(
    thaiid_get_face_image,
    face_image,
    "Face image (base64 JPEG)."
);
c_getter!(
    thaiid_get_laser_id,
    laser_id,
    "Laser-engraved serial number."
);
c_getter!(thaiid_get_main_inscl, main_inscl, "Main insurance scheme.");
c_getter!(thaiid_get_sub_inscl, sub_inscl, "Sub insurance scheme.");
c_getter!(
    thaiid_get_main_hospital,
    main_hospital,
    "Primary hospital name."
);
c_getter!(
    thaiid_get_sub_hospital,
    sub_hospital,
    "Secondary hospital name."
);
c_getter!(thaiid_get_paid_type, paid_type, "Payment type.");
c_getter!(
    thaiid_get_nhso_issue_date,
    nhso_issue_date,
    "NHSO coverage start date."
);
c_getter!(
    thaiid_get_nhso_expire_date,
    nhso_expire_date,
    "NHSO coverage end date."
);
c_getter!(
    thaiid_get_nhso_update_date,
    nhso_update_date,
    "NHSO last update date."
);
c_getter!(
    thaiid_get_change_hospital_amount,
    change_hospital_amount,
    "Hospital change count."
);

fn into_handle(data: model::CardData) -> *mut ThaiIdCardData {
    let personal = data.personal.unwrap_or_else(|| model::Personal {
        cid: String::new(),
        name: model::Name {
            prefix: String::new(),
            first_name: String::new(),
            middle_name: String::new(),
            last_name: String::new(),
            full_name: String::new(),
        },
        name_en: model::Name {
            prefix: String::new(),
            first_name: String::new(),
            middle_name: String::new(),
            last_name: String::new(),
            full_name: String::new(),
        },
        dob: String::new(),
        gender: String::new(),
        card_issuer: String::new(),
        issue_date: String::new(),
        expire_date: String::new(),
        address: model::Address {
            house_no: String::new(),
            moo: String::new(),
            soi: String::new(),
            street: String::new(),
            subdistrict: String::new(),
            district: String::new(),
            province: String::new(),
            full_address: String::new(),
        },
        face_image: String::new(),
    });

    let card_info = data.card.unwrap_or(model::Card {
        laser_id: String::new(),
    });
    let nhso_info = data.nhso.unwrap_or(model::Nhso {
        main_inscl: String::new(),
        sub_inscl: String::new(),
        main_hospital: String::new(),
        sub_hospital: String::new(),
        paid_type: String::new(),
        issue_date: String::new(),
        expire_date: String::new(),
        update_date: String::new(),
        change_hospital_amount: String::new(),
    });

    let handle = ThaiIdCardData {
        cid: to_cstr(&personal.cid),
        name_thai: to_cstr(&personal.name.full_name),
        name_en: to_cstr(&personal.name_en.full_name),
        dob: to_cstr(&personal.dob),
        gender: to_cstr(&personal.gender),
        card_issuer: to_cstr(&personal.card_issuer),
        issue_date: to_cstr(&personal.issue_date),
        expire_date: to_cstr(&personal.expire_date),
        address: to_cstr(&personal.address.full_address),
        face_image: to_cstr(&personal.face_image),
        laser_id: to_cstr(&card_info.laser_id),
        main_inscl: to_cstr(&nhso_info.main_inscl),
        sub_inscl: to_cstr(&nhso_info.sub_inscl),
        main_hospital: to_cstr(&nhso_info.main_hospital),
        sub_hospital: to_cstr(&nhso_info.sub_hospital),
        paid_type: to_cstr(&nhso_info.paid_type),
        nhso_issue_date: to_cstr(&nhso_info.issue_date),
        nhso_expire_date: to_cstr(&nhso_info.expire_date),
        nhso_update_date: to_cstr(&nhso_info.update_date),
        change_hospital_amount: to_cstr(&nhso_info.change_hospital_amount),
    };

    Box::into_raw(Box::new(handle))
}

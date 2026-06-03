//! Data types for Thai National ID smart card data.

/// All data read from a Thai National ID smart card.
#[derive(Debug, Clone)]
pub struct CardData {
    pub personal: Option<Personal>,
    pub card: Option<Card>,
    pub nhso: Option<Nhso>,
}

/// Citizen's personal identification information.
#[derive(Debug, Clone)]
pub struct Personal {
    pub cid: String,
    pub name: Name,
    pub name_en: Name,
    pub dob: String,
    pub gender: String,
    pub card_issuer: String,
    pub issue_date: String,
    pub expire_date: String,
    pub address: Address,
    pub face_image: String,
}

/// Citizen's name broken into components.
#[derive(Debug, Clone)]
pub struct Name {
    pub prefix: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub full_name: String,
}

/// Registered address split into components.
#[derive(Debug, Clone)]
pub struct Address {
    pub house_no: String,
    pub moo: String,
    pub soi: String,
    pub street: String,
    pub subdistrict: String,
    pub district: String,
    pub province: String,
    pub full_address: String,
}

/// Laser-engraved card serial number.
#[derive(Debug, Clone)]
pub struct Card {
    pub laser_id: String,
}

/// NHSO (National Health Security Office) insurance data.
#[derive(Debug, Clone)]
pub struct Nhso {
    pub main_inscl: String,
    pub sub_inscl: String,
    pub main_hospital: String,
    pub sub_hospital: String,
    pub paid_type: String,
    pub issue_date: String,
    pub expire_date: String,
    pub update_date: String,
    pub change_hospital_amount: String,
}

/// Parse a "#"-delimited name string into a `Name`.
pub fn parse_name(raw: &str) -> Name {
    let parts: Vec<&str> = raw.split('#').collect();
    let prefix = parts.first().unwrap_or(&"").to_string();
    let first_name = parts.get(1).unwrap_or(&"").to_string();
    let middle_name = parts.get(2).unwrap_or(&"").to_string();
    let last_name = parts.get(3).unwrap_or(&"").to_string();

    let full_name = if middle_name.is_empty() {
        format!("{}{} {}", prefix, first_name, last_name)
    } else {
        format!("{}{} {} {}", prefix, first_name, middle_name, last_name)
    };

    Name {
        prefix,
        first_name,
        middle_name,
        last_name,
        full_name,
    }
}

/// Parse a "#"-delimited address string into an `Address`.
pub fn parse_address(raw: &str) -> Address {
    let parts: Vec<&str> = raw.split('#').collect();
    let mut addr = Address {
        house_no: String::new(),
        moo: String::new(),
        soi: String::new(),
        street: String::new(),
        subdistrict: String::new(),
        district: String::new(),
        province: String::new(),
        full_address: String::new(),
    };

    if parts.is_empty() {
        return addr;
    }

    addr.house_no = parts[0].to_string();

    // Parse moo (village number) or soi (lane) from the second component
    if parts.len() > 1 {
        let trimmed = parts[1].trim();
        if let Some(val) = trimmed.strip_prefix("หมู่ที่") {
            addr.moo = val.trim().to_string();
        }
        if let Some(val) = trimmed.strip_prefix("ซอย") {
            addr.soi = val.trim().to_string();
        }
    }

    // Street is everything between index 2 and the last 3 items
    if parts.len() > 4 {
        let street_parts: Vec<&str> = parts[2..parts.len() - 3].to_vec();
        addr.street = street_parts.join(" ").trim().to_string();
    }

    // Subdistrict (ตำบล or แขวง)
    if parts.len() > 3 {
        let sd = parts[parts.len() - 3].trim();
        addr.subdistrict = sd
            .strip_prefix("ตำบล")
            .or_else(|| sd.strip_prefix("แขวง"))
            .unwrap_or(sd)
            .trim()
            .to_string();
    }

    // District (อำเภอ or เขต)
    if parts.len() > 2 {
        let d = parts[parts.len() - 2].trim();
        addr.district = d
            .strip_prefix("อำเภอ")
            .or_else(|| d.strip_prefix("เขต"))
            .unwrap_or(d)
            .trim()
            .to_string();
    }

    // Province (จังหวัด)
    if let Some(last) = parts.last() {
        let p = last.trim();
        addr.province = p.strip_prefix("จังหวัด").unwrap_or(p).trim().to_string();
    }

    let non_empty: Vec<&str> = parts.iter().filter(|s| !s.is_empty()).copied().collect();
    addr.full_address = non_empty.join(" ");

    addr
}

/// Convert a raw "YYYYMMDD" (Buddhist year) date to "YYYY-MM-DD" (Gregorian).
pub fn format_date(raw: &str) -> String {
    if raw.len() != 8 {
        return String::new();
    }
    let year: i32 = raw[0..4].parse().unwrap_or(0);
    let month = &raw[4..6];
    let day = &raw[6..8];
    format!("{:04}-{}-{}", year - 543, month, day)
}

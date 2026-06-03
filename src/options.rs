//! Options for configuring card reads.

/// Controls which data sections are read from the Thai National ID card.
/// When `reader_name` is `None`, the first available reader with a card is used.
/// Face images are returned as a base64-encoded JPEG string.
#[derive(Debug, Clone)]
pub struct Options {
    pub reader_name: Option<String>,
    pub show_face_image: bool,
    pub show_nhso_data: bool,
    pub show_laser_data: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            reader_name: None,
            show_face_image: true,
            show_nhso_data: false,
            show_laser_data: false,
        }
    }
}

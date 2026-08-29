//! Definisi format file yang didukung beserta magic bytes-nya.

/// Suatu format file beserta magic bytes khasnya.
#[derive(Debug, Clone, Copy)]
pub struct Format {
    /// Daftar ekstensi (tanpa titik) yang dipetakan ke format ini.
    pub extensions: &'static [&'static str],
    /// Byte pembuka khas format (signature).
    pub magic: &'static [u8],
}

impl Format {
    /// Mencari format berdasarkan ekstensi (tanpa titik).
    ///
    /// Mengembalikan `None` jika ekstensi tidak dikenali.
    pub fn from_extension(ext: &str) -> Option<&'static Self> {
        FORMATS
            .iter()
            .find(|format| format.extensions.contains(&ext))
    }
}

/// Daftar seluruh format yang didukung beserta magic bytes-nya.
///
/// `docx` dan `xlsx` adalah arsip ZIP sehingga berbagi signature yang sama
/// dengan `zip`.
#[rustfmt::skip]
pub const FORMATS: &[Format] = &[
    Format {
        extensions: &["mp4"],
        magic: &[
            0x00, 0x00, 0x00, 0x20, 0x66, 0x74, 0x79, 0x70,
            0x69, 0x73, 0x6F, 0x6D, 0x00, 0x00, 0x02, 0x00,
            0x69, 0x73, 0x6F, 0x6D, 0x69, 0x73, 0x6F, 0x32,
            0x61, 0x76, 0x63, 0x31, 0x64, 0x61, 0x74, 0x61,
        ],
    },
    Format {
        extensions: &["mkv"],
        magic: &[
            0x1A, 0x45, 0xDF, 0xA3, 0x93, 0x42, 0x82, 0x88,
            0x6D, 0x61, 0x74, 0x72, 0x6F, 0x73, 0x6B, 0x61,
        ],
    },
    Format {
        extensions: &["mp3"],
        magic: &[0x49, 0x44, 0x33, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x21],
    },
    Format {
        extensions: &["wav"],
        magic: &[
            0x52, 0x49, 0x46, 0x46, 0x24, 0x00, 0x00, 0x00,
            0x57, 0x41, 0x56, 0x45, 0x66, 0x6D, 0x74, 0x20,
        ],
    },
    Format {
        extensions: &["jpg", "jpeg"],
        magic: &[0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46],
    },
    Format {
        extensions: &["png"],
        magic: &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A],
    },
    Format {
        extensions: &["gif"],
        magic: b"GIF89a",
    },
    Format {
        extensions: &["webp"],
        magic: &[
            0x52, 0x49, 0x46, 0x46, 0x24, 0x00, 0x00, 0x00,
            0x57, 0x45, 0x42, 0x50, 0x56, 0x50, 0x38, 0x20,
        ],
    },
    Format {
        extensions: &["pdf"],
        magic: b"%PDF-1.4\n",
    },
    Format {
        extensions: &["zip", "docx", "xlsx"],
        magic: &[0x50, 0x4B, 0x03, 0x04, 0x14, 0x00, 0x00, 0x00],
    },
    Format {
        extensions: &["rar"],
        magic: &[0x52, 0x61, 0x72, 0x21, 0x1A, 0x07, 0x01, 0x00],
    },
    Format {
        extensions: &["7z"],
        magic: &[0x37, 0x7A, 0xBC, 0xAF, 0x27, 0x1C],
    },
];

/// Daftar seluruh ekstensi yang didukung, dipisahkan koma untuk ditampilkan
/// pada prompt.
pub fn supported_extensions() -> String {
    FORMATS
        .iter()
        .flat_map(|format| format.extensions.iter().copied())
        .collect::<Vec<_>>()
        .join(", ")
}

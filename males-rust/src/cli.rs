//! Interaksi dengan pengguna: prompt input dan penguraian ukuran.

use std::io::{self, Write};

use crate::magic;

/// Keseluruhan masukan dari pengguna: nama, ekstensi, dan ukuran.
#[derive(Debug)]
pub struct UserInput {
    /// Nama file tanpa ekstensi.
    pub stem: String,
    /// Ekstensi (tanpa titik).
    pub extension: String,
    /// Ukuran file dalam byte.
    pub size: u64,
}

impl UserInput {
    /// Nama lengkap file beserta ekstensinya.
    pub fn filename(&self) -> String {
        format!("{}.{}", self.stem, self.extension)
    }
}

/// Membaca satu baris masukan dari pengguna setelah menampilkan `prompt`.
fn read_line(prompt: &str) -> io::Result<String> {
    print!("{prompt}");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

/// Meminta seluruh masukan dari pengguna.
pub fn collect() -> io::Result<UserInput> {
    let stem = read_line("Nama file (tanpa ekstensi): ")?;
    let stem = if stem.is_empty() { "dummy" } else { &stem };

    let extension =
        read_line(&format!("Format ({}): ", magic::supported_extensions()))?.to_lowercase();

    let size_input = read_line("Ukuran (contoh: 5M, 10K, 100): ")?;
    let size = parse_size(&size_input)?;

    Ok(UserInput {
        stem: stem.to_string(),
        extension,
        size,
    })
}

/// Mengurai ukuran dari string seperti `5M`, `10K`, `1G`, atau `100`
/// (satuan kelipatan 1024).
pub fn parse_size(input: &str) -> io::Result<u64> {
    let input = input.trim().to_uppercase();
    let (num_str, multiplier) = if input.ends_with('K') {
        (&input[..input.len() - 1], 1024)
    } else if input.ends_with('M') {
        (&input[..input.len() - 1], 1024 * 1024)
    } else if input.ends_with('G') {
        (&input[..input.len() - 1], 1024 * 1024 * 1024)
    } else {
        (input.as_str(), 1)
    };

    let base: u64 = num_str.parse().map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Format ukuran tidak valid: {e}"),
        )
    })?;

    Ok(base * multiplier)
}

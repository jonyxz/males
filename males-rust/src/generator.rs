//! Pembuatan file: alokasi ukuran, pengisian pseudo-acak, dan penyisipan
//! magic bytes di awal file.

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};

use crate::magic::Format;
use crate::rng::SimpleRng;

/// Ukuran buffer per iterasi saat mengisi file (1 MiB).
const CHUNK_SIZE: usize = 1024 * 1024;

/// Membuat file berisi data pseudo-acak sebesar `size` byte dengan nama
/// `filename`.
///
/// Jika `format` diberikan (ekstensi dikenali), magic bytes ditulis di awal
/// file setelah pengisian selesai.
pub fn generate_corrupt_file(
    filename: &str,
    size: u64,
    format: Option<&Format>,
) -> io::Result<u64> {
    let written = fill_with_random(filename, size)?;

    if let Some(format) = format {
        write_magic_bytes(filename, format.magic)?;
    }

    Ok(written)
}

/// Mengisi file baru dengan data pseudo-acak hingga `size` byte.
fn fill_with_random(filename: &str, size: u64) -> io::Result<u64> {
    let mut file = File::create(filename)?;
    let mut rng = SimpleRng::new();
    let mut buffer = vec![0u8; CHUNK_SIZE];
    let mut written = 0u64;

    while written < size {
        let to_write = (size - written).min(CHUNK_SIZE as u64) as usize;
        rng.read_exact(&mut buffer[..to_write])?;
        file.write_all(&buffer[..to_write])?;
        written += to_write as u64;
    }

    Ok(written)
}

/// Menulis `magic` di awal file yang sudah dibuat tanpa mengubah isi lainnya.
fn write_magic_bytes(path: &str, magic: &[u8]) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).open(path)?;
    file.seek(SeekFrom::Start(0))?;
    file.write_all(magic)
}

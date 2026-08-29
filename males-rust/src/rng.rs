//! Generator bilangan pseudo-acak berbasis xorshift64.
//!
//! Tidak bergantung pada sumber keacakan sistem (seperti `/dev/urandom`),
//! sehingga kompilasi dan berjalan konsisten di Linux, macOS, dan Windows.

use std::io::{self, Read};

/// Generator sederhana deterministik yang mengisi buffer dengan byte acak.
pub struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    /// Membuat generator dengan seed default.
    pub fn new() -> Self {
        SimpleRng {
            state: 0x853c_49e6_748f_ea9b,
        }
    }
}

impl Default for SimpleRng {
    fn default() -> Self {
        Self::new()
    }
}

impl Read for SimpleRng {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        for byte in buf.iter_mut() {
            self.state ^= self.state << 13;
            self.state ^= self.state >> 7;
            self.state ^= self.state << 17;
            *byte = self.state as u8;
        }
        Ok(buf.len())
    }
}

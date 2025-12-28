use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;

/// cbindgen:ignore
const PRISM_FORMAT_PREFIX: &str = "Rime::Prism/";
/// cbindgen:ignore
const PRISM_FORMAT_MAX_LEN: usize = 32; // prism::Metadata::kFormatMaxLength
/// cbindgen:ignore
const PRISM_ALPHABET_LEN: usize = 256;

#[derive(Debug, Clone, PartialEq)]
pub struct PrismInfo {
    pub format: String,
    pub version: f32,
    pub dict_file_checksum: u32,
    pub schema_file_checksum: u32,
    pub num_syllables: u32,
    pub num_spellings: u32,
    pub double_array_size: u32,
    pub double_array_offset_abs: Option<usize>,
    pub has_spelling_map: bool,
    pub alphabet: String,
}

pub fn parse_prism_bytes(bytes: &[u8]) -> Result<PrismInfo> {
    // Metadata is at file offset 0.
    if bytes.len() < PRISM_FORMAT_MAX_LEN + 4 * 5 + 4 + 4 + PRISM_ALPHABET_LEN {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "file too small for Prism metadata",
        ));
    }

    let format_raw = &bytes[0..PRISM_FORMAT_MAX_LEN];
    let nul_pos = format_raw
        .iter()
        .position(|&b| b == 0)
        .unwrap_or(PRISM_FORMAT_MAX_LEN);
    let format = String::from_utf8_lossy(&format_raw[..nul_pos]).to_string();
    if !format.starts_with(PRISM_FORMAT_PREFIX) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "invalid Prism format prefix",
        ));
    }
    let version_str = &format[PRISM_FORMAT_PREFIX.len()..];
    let version: f32 = version_str.parse().unwrap_or(0.0);

    // little-endian u32 fields
    let read_u32 =
        |off: usize| -> u32 { u32::from_le_bytes(bytes[off..off + 4].try_into().unwrap()) };

    let mut cursor = PRISM_FORMAT_MAX_LEN;
    let dict_file_checksum = read_u32(cursor);
    cursor += 4;
    let schema_file_checksum = read_u32(cursor);
    cursor += 4;
    let num_syllables = read_u32(cursor);
    cursor += 4;
    let num_spellings = read_u32(cursor);
    cursor += 4;
    let double_array_size = read_u32(cursor);
    cursor += 4;

    let double_array_offset_rel = i32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap());
    let double_array_offset_abs = if double_array_offset_rel == 0 {
        None
    } else {
        let ptr_pos = cursor; // position of the offset field in the file
        let abs = ptr_pos as isize + double_array_offset_rel as isize;
        let abs_usize = abs as usize;
        if abs_usize >= bytes.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "double_array offset out of bounds",
            ));
        }
        Some(abs_usize)
    };
    cursor += 4;

    let spelling_map_offset_rel = i32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap());
    let has_spelling_map = spelling_map_offset_rel != 0;
    cursor += 4;

    let alphabet_raw = &bytes[cursor..cursor + PRISM_ALPHABET_LEN];
    let alpha_nul = alphabet_raw
        .iter()
        .position(|&b| b == 0)
        .unwrap_or(PRISM_ALPHABET_LEN);
    let alphabet = String::from_utf8_lossy(&alphabet_raw[..alpha_nul]).to_string();

    Ok(PrismInfo {
        format,
        version,
        dict_file_checksum,
        schema_file_checksum,
        num_syllables,
        num_spellings,
        double_array_size,
        double_array_offset_abs,
        has_spelling_map,
        alphabet,
    })
}

pub fn parse_prism_file<P: AsRef<Path>>(path: P) -> Result<PrismInfo> {
    let mut f = File::open(path)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    parse_prism_bytes(&buf)
}

#[cfg(test)]
mod tests;

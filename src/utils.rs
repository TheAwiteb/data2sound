use crate::{Error, Result, SAMPLE_RATE};
use std::fs;

/// Check the file size. The maximum file size is 4 GB.
pub(crate) fn check_file_size(file: &fs::File) -> Result<()> {
    (file.metadata()?.len() > 4_294_967_295)
        .then(|| Err(Error::LargeFileSize))
        .unwrap_or(Ok(()))
}

/// Check if the wav file size is valid, the minimum size is 44 bytes. (the maximum will checked with `check_file_size`)
/// ### Note
/// The 44 bytes are the header of the wav file.
pub(crate) fn check_wav_file_size(file: &fs::File) -> Result<()> {
    (file.metadata()?.len() < 44)
        .then(|| {
            Err(Error::InvalidWavFile(
                "the minimum size is 44 bytes".to_owned(),
            ))
        })
        .unwrap_or(Ok(()))
}

/// Create the wave file header, the header is 44 bytes.
/// ### Note
/// - The `file_size` is the size of the file that will be encoded.
/// - The bits per sample is 16.
/// - The sample rate is 202860 Hz (202.86 kHz).
/// - The number of channels is 1.
pub(crate) fn create_wav_header(file_size: u32) -> [u8; 44] {
    let mut header = [0; 44];
    header[0..4].copy_from_slice(b"RIFF");
    header[4..8].copy_from_slice(&(file_size + 36).to_le_bytes());
    header[8..12].copy_from_slice(b"WAVE");
    header[12..16].copy_from_slice(b"fmt ");
    header[16..20].copy_from_slice(&16u32.to_le_bytes());
    header[20..22].copy_from_slice(&1u16.to_le_bytes());
    header[22..24].copy_from_slice(&1u16.to_le_bytes());
    header[24..28].copy_from_slice(&SAMPLE_RATE.to_le_bytes());
    header[28..32].copy_from_slice(&(SAMPLE_RATE * 2).to_le_bytes());
    header[32..34].copy_from_slice(&2u16.to_le_bytes());
    header[34..36].copy_from_slice(&16u16.to_le_bytes());
    header[36..40].copy_from_slice(b"data");
    header[40..44].copy_from_slice(&file_size.to_le_bytes());
    header
}

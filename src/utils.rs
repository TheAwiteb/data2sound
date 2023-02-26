use crate::{Error, Result};
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

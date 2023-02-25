use crate::{Error, Result};
use std::fs;

/// Check the file size. The maximum file size is 4 GB.
pub(crate) fn check_file_size(file: &fs::File) -> Result<()> {
    (file.metadata()?.len() > 4_294_967_295)
        .then(|| Err(Error::LargeFileSize))
        .unwrap_or(Ok(()))
}

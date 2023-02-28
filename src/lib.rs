#![doc = include_str!("../README.md")]

use std::{
    fs,
    io::{self, BufReader, BufWriter, Seek, Write},
    path,
};

mod errors;
mod utils;
pub use errors::*;

/// The sample rate to use when encoding
pub const SAMPLE_RATE: u32 = 202860;

/// Encode file to wav file by converting the bytes to a sine wave, then writing the sine wave to a wav file
/// # Arguments
/// * `file` - The file to encode
/// * `wav_output` - The wav file to write the sine wave to
/// # Example
/// ```rust|no_run
/// use data2sound::encode;
/// encode("test.txt", "test.wav").unwrap();
/// ```
pub fn encode(file: impl AsRef<path::Path>, wav_output: impl AsRef<path::Path>) -> Result<()> {
    let file = fs::File::open(file)?;
    utils::check_file_size(&file)?;
    let str_path = wav_output.as_ref().display().to_string();
    let wav_output = if !str_path.ends_with(".wav") {
        format!("{}.wav", wav_output.as_ref().display())
    } else {
        str_path
    };
    let mut reader = BufReader::new(&file);
    let mut writer = BufWriter::new(fs::File::create(wav_output)?);
    // Write the wav header to the wav file
    writer.write_all(&utils::create_wav_header(file.metadata()?.len() as u32))?;
    // Copy the file to the wav file
    io::copy(&mut reader, &mut writer)?;
    Ok(())
}

/// Decode wav file to file by converting the sine wave to bytes
/// # Arguments
/// * `file` - The wav file to decode
/// * `output` - The file to write the bytes to
/// # Example
/// ```rust|no_run
/// use data2sound::decode;
/// use std::fs;
/// decode("test.wav", "test.txt").unwrap();
/// ```
pub fn decode(file: impl AsRef<path::Path>, output: impl AsRef<path::Path>) -> Result<()> {
    let output_file = fs::File::open(&file)?;
    utils::check_file_size(&output_file)?;
    utils::check_wav_file_size(&output_file)?;
    let mut reader = BufReader::new(output_file);
    let mut writer = BufWriter::new(fs::File::create(output)?);
    // Skip the header, to get to the data (the header is 44 bytes long)
    reader.seek(std::io::SeekFrom::Start(44))?;
    // Copy the wave file after skipping the header to the output file
    io::copy(&mut reader, &mut writer)?;
    Ok(())
}

/// Encode the given bytes to a wav bytes vector
/// # Arguments
/// * `bytes` - The bytes to encode
/// # Example
/// ```rust
/// use data2sound::encode_bytes;
/// let bytes = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
/// let wav_bytes = encode_bytes(&bytes).unwrap();
/// assert_eq!(wav_bytes.len(), 44 + bytes.len());
/// assert_eq!(wav_bytes[44..], bytes[..]);
/// ```
pub fn encode_bytes(bytes: &[u8]) -> Result<Vec<u8>> {
    if bytes.len() > 4294967295 {
        return Err(Error::LargeFileSize);
    }
    let mut wav_bytes = utils::create_wav_header(bytes.len() as u32).to_vec();
    wav_bytes.extend_from_slice(bytes);
    Ok(wav_bytes)
}

/// Decode the given wav bytes to bytes
/// # Arguments
/// * `bytes` - The wav bytes to decode
/// # Example
/// ```rust
/// use data2sound::decode_bytes;
/// let bytes = vec![82, 73, 70, 70, 46, 0, 0, 0, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0, 1, 0, 108, 24, 3, 0, 216, 48, 6, 0, 2, 0, 16, 0, 100, 97, 116, 97, 10, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
/// let decoded_bytes = decode_bytes(&bytes).unwrap();
/// assert_eq!(decoded_bytes.len(), 10);
/// assert_eq!(decoded_bytes, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
/// ```
pub fn decode_bytes(bytes: &[u8]) -> Result<Vec<u8>> {
    if bytes.len() < 44 {
        return Err(Error::InvalidWavFile(
            "the minimum wav file size is 44 bytes".to_string(),
        ));
    }
    Ok(bytes[44..].to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_encode_decode() {
        fs::write("test.txt", "Some context").unwrap();
        encode("test.txt", "test.wav").unwrap();
        decode("test.wav", "test2.txt").unwrap();
        let file = fs::File::open("test.txt").unwrap();
        let file2 = fs::File::open("test2.txt").unwrap();
        assert_eq!(
            fs::read_to_string("test.txt").unwrap(),
            fs::read_to_string("test2.txt").unwrap()
        );
        assert_eq!(
            file.metadata().unwrap().len(),
            file2.metadata().unwrap().len()
        );
        fs::remove_file("test.txt").unwrap();
        fs::remove_file("test2.txt").unwrap();
        fs::remove_file("test.wav").unwrap();
    }
}

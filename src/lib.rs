use file_utils::read::Read;
use std::{
    fs,
    io::{self, BufReader, BufWriter, Seek},
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
/// use std::fs;
/// let file = fs::File::open("test.txt").unwrap();
/// encode(file, "test.wav").unwrap();
/// ```
pub fn encode(file: fs::File, wav_output: impl AsRef<path::Path>) -> Result<()> {
    utils::check_file_size(&file)?;
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let str_path = wav_output.as_ref().display().to_string();
    let wav_output = if !str_path.ends_with(".wav") {
        format!("{}.wav", wav_output.as_ref().display())
    } else {
        str_path
    };
    let mut writer = hound::WavWriter::create(wav_output, spec)?;

    let mut file = BufReader::new(file);

    while let Ok(byte) = file.read_i16() {
        writer.write_sample(byte)?;
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_encode_decode() {
        fs::write("test.txt", "Some context").unwrap();
        encode(fs::File::open("test.txt").unwrap(), "test.wav").unwrap();
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

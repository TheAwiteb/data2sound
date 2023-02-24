use file_utils::{read::Read, write::Write};
use std::{
    fs,
    io::{BufReader, BufWriter},
    path,
};

/// The sample rate to use when encoding
pub const SAMPLE_RATE: u32 = 202860;
/// The result type for this crate, which is just a re-export of `hound::Result`
pub use hound::Result;

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
    let mut reader = hound::WavReader::open(file.as_ref()).unwrap();
    let mut writer = BufWriter::new(fs::File::create(output).unwrap());
    for sample in reader.samples() {
        writer.write_i16(sample?)?;
    }
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

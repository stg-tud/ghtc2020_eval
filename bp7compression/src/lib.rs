use bzip2::read::BzEncoder;
use bzip2::Compression;
use core::num::ParseIntError;
use miniz_oxide::deflate::compress_to_vec;
use std::io::Read;
use std::io::Write;
use xz2::read::XzEncoder;

/// Convert a hex string into a byte vector
pub fn unhexify(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}
pub fn libflate_gzip_compress(indata: &[u8]) -> Vec<u8> {
    let mut encoder = libflate::gzip::Encoder::new(Vec::new()).unwrap();
    encoder.write_all(indata).unwrap();
    encoder.finish().into_result().unwrap()
}

pub fn libflate_deflate_compress(indata: &[u8]) -> Vec<u8> {
    let mut encoder = libflate::deflate::Encoder::new(Vec::new());
    encoder.write_all(indata).unwrap();
    encoder.finish().into_result().unwrap()
}
pub fn libflate_zlib_compress(indata: &[u8]) -> Vec<u8> {
    let mut encoder = libflate::zlib::Encoder::new(Vec::new()).unwrap();
    encoder.write_all(indata).unwrap();
    encoder.finish().into_result().unwrap()
}

pub fn snap_compress(indata: &[u8]) -> Vec<u8> {
    let mut encoder = snap::Encoder::new();
    encoder.compress_vec(indata).unwrap()
}
pub fn bz2_compress(indata: &[u8]) -> Vec<u8> {
    let mut compressor = BzEncoder::new(indata, Compression::Best);
    let mut buf = Vec::new();
    compressor.read_to_end(&mut buf);
    buf
}
pub fn brotli_compress(indata: &[u8]) -> Vec<u8> {
    let buf: Vec<u8> = Vec::new();
    let mut writer = brotli::CompressorWriter::new(buf, 4096, 11, 22);
    writer.write_all(indata).unwrap();
    writer.into_inner()
}
pub fn miniz_compress(indata: &[u8]) -> Vec<u8> {
    compress_to_vec(indata, 10)
}
pub fn xz2_compress(indata: &[u8]) -> Vec<u8> {
    let mut compressor = XzEncoder::new(indata, 9);
    let mut buf = Vec::new();
    compressor.read_to_end(&mut buf);
    buf
}

pub fn smaz_compress(indata: &[u8]) -> Vec<u8> {
    smaz::compress(indata)
}

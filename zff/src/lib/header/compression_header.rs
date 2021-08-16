// - internal
use crate::{
	HeaderObject,
	HeaderEncoder,
	ValueType,
};

use crate::{
	HEADER_IDENTIFIER_COMPRESSION_HEADER,
};

#[derive(Debug,Clone)]
pub struct CompressionHeader {
	header_version: u8,
	compression_algorithm: CompressionAlgorithm,
	compression_level: u8
}

impl CompressionHeader {
	pub fn new(header_version: u8, compression_algo: CompressionAlgorithm, compression_level: u8) -> CompressionHeader {
		Self {
			header_version: header_version,
			compression_algorithm: compression_algo,
			compression_level: compression_level,
		}
	}

	pub fn header_version(&self) -> &u8 {
		&self.header_version
	}
	pub fn compression_algorithm(&self) -> &CompressionAlgorithm {
		&self.compression_algorithm
	}
	pub fn compression_level(&self) -> &u8 {
		&self.compression_level
	}
}

impl HeaderObject for CompressionHeader {
	fn identifier() -> u32 {
		HEADER_IDENTIFIER_COMPRESSION_HEADER
	}
	fn encode_header(&self) -> Vec<u8> {
		let mut vec = Vec::new();

		vec.push(self.header_version);
		vec.push(self.compression_algorithm.get_value());
		vec.push(self.compression_level);
		
		vec
	}
}

impl HeaderEncoder for CompressionHeader {
	fn encode_directly(&self) -> Vec<u8> {
		let mut vec = Vec::new();
		let mut encoded_header = self.encode_header();
		let identifier = Self::identifier();
		let encoded_header_length = encoded_header.len() as u64;
		vec.append(&mut identifier.to_be_bytes().to_vec());
		vec.append(&mut encoded_header_length.to_le_bytes().to_vec());
		vec.append(&mut encoded_header);

		vec
	}
	fn encode_for_key<K: Into<String>>(&self, key: K) -> Vec<u8> {
		let mut vec = Vec::new();
		let mut encoded_key = Self::encode_key(key);
		vec.append(&mut encoded_key);
		vec.push(ValueType::Object.as_raw_value());
		vec.append(&mut self.encode_directly());
		vec
	}
}

#[derive(Debug,Clone)]
pub enum CompressionAlgorithm {
	None,
	Zstd,
}

impl CompressionAlgorithm {
	pub fn get_value(&self) -> u8 {
		match self {
			CompressionAlgorithm::None => 0,
			CompressionAlgorithm::Zstd => 1,
		}
	}
}
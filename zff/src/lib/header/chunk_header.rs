// - external
use ed25519_dalek::{SIGNATURE_LENGTH};

// - internal
use crate::{
	HeaderEncoder,
	HeaderObject,
	HEADER_IDENTIFIER_CHUNK_HEADER,
};

#[derive(Debug,Clone)]
pub struct ChunkHeader {
	header_version: u8,
	chunk_number: u64,
	chunk_size: u64,
	crc32: u32,
	ed25519_signature: Option<[u8; SIGNATURE_LENGTH]>,
}

impl ChunkHeader {
	pub fn new(header_version: u8, chunk_number: u64, chunk_size: u64, crc32: u32, ed25519_signature: Option<[u8; SIGNATURE_LENGTH]>) -> ChunkHeader {
		Self {
			header_version: header_version,
			chunk_number: chunk_number,
			chunk_size: chunk_size,
			crc32: crc32,
			ed25519_signature: ed25519_signature
		}
	}

	pub fn set_chunk_size(&mut self, size: u64) {
		self.chunk_size = size
	}

	pub fn set_crc32(&mut self, crc32: u32) {
		self.crc32 = crc32
	}

	pub fn set_signature(&mut self, signature: Option<[u8; SIGNATURE_LENGTH]>) {
		self.ed25519_signature = signature
	}

	pub fn next_number(&mut self) {
		self.chunk_number += 1;
	}

	pub fn chunk_number(&self) -> u64 {
		self.chunk_number
	}
}

impl HeaderObject for ChunkHeader {
	fn identifier() -> u32 {
		HEADER_IDENTIFIER_CHUNK_HEADER
	}
	fn encode_header(&self) -> Vec<u8> {
		let mut vec = Vec::new();

		vec.push(self.header_version);
		vec.append(&mut self.chunk_number.encode_directly());
		vec.append(&mut self.chunk_size.encode_directly());
		vec.append(&mut self.crc32.encode_directly());
		match self.ed25519_signature {
			None => (),
			Some(signature) => vec.append(&mut signature.encode_directly()),
		};
		
		vec
	}
}

impl HeaderEncoder for ChunkHeader {
	fn encode_directly(&self) -> Vec<u8> {
		let mut vec = Vec::new();
		let mut encoded_header = self.encode_header();
		let identifier = Self::identifier();
		let encoded_header_length = 4 + 8 + (encoded_header.len() as u64); //4 bytes identifier + 8 bytes for length + length itself
		vec.append(&mut identifier.to_be_bytes().to_vec());
		vec.append(&mut encoded_header_length.to_le_bytes().to_vec());
		vec.append(&mut encoded_header);

		vec
	}
	fn encode_for_key<K: Into<String>>(&self, key: K) -> Vec<u8> {
		let mut vec = Vec::new();
		let mut encoded_key = Self::encode_key(key);
		vec.append(&mut encoded_key);
		vec.append(&mut self.encode_directly());
		vec
	}
}
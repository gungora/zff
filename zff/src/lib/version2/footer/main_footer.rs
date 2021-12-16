// - STD
use std::io::Cursor;

// - internal
use crate::{
	Result,
	HeaderCoding,
	ValueDecoder,
	ValueEncoder,
	FOOTER_IDENTIFIER_MAIN_FOOTER,
};

/// The main footer is the last thing, which is written at the end of the last segment.\
/// This footer contains a lot of variable information (e.g. number of segments, ...).
#[derive(Debug,Clone)]
pub struct MainFooter {
	version: u8,
	number_of_segments: u64,
	number_of_objects: u64,
	/// offset in the current segment, where the footer starts.
	footer_offset: u64,
}

impl MainFooter {
	pub fn new(version: u8, number_of_segments: u64, number_of_objects: u64, footer_offset: u64) -> MainFooter {
		Self {
			version: version,
			number_of_segments: number_of_segments,
			number_of_objects: number_of_objects,
			footer_offset: footer_offset,
		}
	}

	pub fn version(&self) -> u8 {
		self.version
	}
	pub fn number_of_segments(&self) -> u64 {
		self.number_of_segments
	}
	pub fn number_of_objects(&self) -> u64 {
		self.number_of_objects
	}
	pub fn footer_offset(&self) -> u64 {
		self.footer_offset
	}
}

impl HeaderCoding for MainFooter {
	type Item = MainFooter;

	fn identifier() -> u32 {
		FOOTER_IDENTIFIER_MAIN_FOOTER
	}

	fn version(&self) -> u8 {
		self.version
	}

	fn encode_header(&self) -> Vec<u8> {
		let mut vec = Vec::new();
		vec.append(&mut self.version.encode_directly());
		vec.append(&mut self.number_of_segments.encode_directly());
		vec.append(&mut self.number_of_objects.encode_directly());
		vec.append(&mut self.footer_offset.encode_directly());

		vec
	}

	fn decode_content(data: Vec<u8>) -> Result<MainFooter> {
		let mut cursor = Cursor::new(data);

		let footer_version = u8::decode_directly(&mut cursor)?;
		let number_of_segments = u64::decode_directly(&mut cursor)?;
		let number_of_objects = u64::decode_directly(&mut cursor)?;
		let footer_offset = u64::decode_directly(&mut cursor)?;
		Ok(MainFooter::new(footer_version, number_of_segments, number_of_objects, footer_offset))
	}
}

//TODO ENCRYPTED MainFooter encoder/decoder?
// - STD
use std::cmp::{PartialEq};
use std::io::Cursor;

// - internal
use crate::{
	Result,
	HeaderObject,
	HeaderEncoder,
	HeaderDecoder,
	ValueEncoder,
	ValueDecoder,
	HEADER_IDENTIFIER_SEGMENT_HEADER
};

// - external
use serde::{Serialize};

/// The segment header contains all informations about the specific segment. Each segment has his own segment header.\
/// This header is **not** a part of the main header.\
/// The segment header has the following layout:\
/// 
/// | Magic <br>bytes | Header<br>length | Header<br>version | Unique<br>identifier | Segment<br>number | Length of the<br>segment |
/// |-----------------|------------------|-------------------|----------------------|-------------------|--------------------------|
/// | 4 bytes         | 8 bytes          | 1 byte            | 8 bytes              | 8 bytes           | 8 bytes                  |
/// | 0x7A666673      | uint64           | uint8             | int64               | uint64            | uint64                   |
#[derive(Debug,Clone,Eq,Serialize)]
pub struct SegmentHeader {
	header_version: u8,
	unique_identifier: i64,
	segment_number: u64,
	length_of_segment: u64,
}

impl SegmentHeader {
	/// returns a new segment header with the given values.
	pub fn new(header_version: u8, unique_identifier: i64, segment_number: u64, length_of_segment: u64) -> SegmentHeader {
		Self {
			header_version: header_version,
			unique_identifier: unique_identifier,
			segment_number: segment_number,
			length_of_segment: length_of_segment,
		}
	}

	/// returns the version of the segment header.
	pub fn header_version(&self) -> u8 {
		self.header_version
	}

	/// returns the unique identifier of image (each segment should have the same identifier).
	pub fn unique_identifier(&self) -> i64 {
		self.unique_identifier
	}

	/// returns the segment number.
	pub fn segment_number(&self) -> u64 {
		self.segment_number
	}

	/// returns the length of the segment in bytes.
	pub fn length_of_segment(&self) -> u64 {
		self.length_of_segment
	}

	/// overwrites the length value in the header with the given value. This can be useful, if you create an 'empty'
	/// header (with length=0) and want to set the length value after reading the data from source to buffer.
	pub fn set_length_of_segment(&mut self, value: u64) {
		self.length_of_segment = value
	}

	/// sets the segment number to the next number. This can be useful, for example,
	/// if you clone a segment header from the previous one or something like that.
	pub fn next_header(&self) -> SegmentHeader {
		SegmentHeader {
			header_version: self.header_version,
			unique_identifier: self.unique_identifier,
			segment_number: self.segment_number+1,
			length_of_segment: 0
		}
	}
}

impl HeaderObject for SegmentHeader {
	fn identifier() -> u32 {
		HEADER_IDENTIFIER_SEGMENT_HEADER
	}
	fn encode_header(&self) -> Vec<u8> {
		let mut vec = Vec::new();

		vec.append(&mut self.header_version.encode_directly());
		vec.append(&mut self.unique_identifier.encode_directly());
		vec.append(&mut self.segment_number.encode_directly());
		vec.append(&mut self.length_of_segment.encode_directly());

		vec
	}
}

impl HeaderEncoder for SegmentHeader {}

impl HeaderDecoder for SegmentHeader {
	type Item = SegmentHeader;

	fn decode_content(data: Vec<u8>) -> Result<SegmentHeader> {
		let mut cursor = Cursor::new(data);

		let header_version = u8::decode_directly(&mut cursor)?;
		let unique_identifier = i64::decode_directly(&mut cursor)?;
		let segment_number = u64::decode_directly(&mut cursor)?;
		let length = u64::decode_directly(&mut cursor)?;
		Ok(SegmentHeader::new(header_version, unique_identifier, segment_number, length))
	}
}

impl PartialEq for SegmentHeader {
    fn eq(&self, other: &Self) -> bool {
        self.segment_number == other.segment_number
    }
}
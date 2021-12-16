use ed25519_dalek::{PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH};

// identifier: magic bytes
pub(crate) const HEADER_IDENTIFIER_MAIN_HEADER: u32 = 0x7A66666d;
pub(crate) const HEADER_IDENTIFIER_ENCRYPTED_MAIN_HEADER: u32 = 0x7a666645;
pub(crate) const HEADER_IDENTIFIER_DESCRIPTION_HEADER: u32 = 0x7A666664;
pub(crate) const HEADER_IDENTIFIER_SEGMENT_HEADER: u32 = 0x7A666673;
pub(crate) const HEADER_IDENTIFIER_COMPRESSION_HEADER: u32 = 0x7A666663;
pub(crate) const HEADER_IDENTIFIER_PBE_HEADER: u32 = 0x7A666670;
pub(crate) const HEADER_IDENTIFIER_ENCRYPTION_HEADER: u32 = 0x7A666665;
pub(crate) const HEADER_IDENTIFIER_CHUNK_HEADER: u32 = 0x7A666643;
pub(crate) const HEADER_IDENTIFIER_HASH_HEADER: u32 = 0x7a666668;
pub(crate) const HEADER_IDENTIFIER_HASH_VALUE: u32 = 0x7a666648;
pub(crate) const HEADER_IDENTIFIER_OBJECT_HEADER: u32 = 0x7a66664f;
pub(crate) const HEADER_IDENTIFIER_FILE_HEADER: u32 = 0x7A666666;

pub(crate) const FOOTER_IDENTIFIER_SEGMENT_FOOTER: u32 = 0x7A666646;
pub(crate) const FOOTER_IDENTIFIER_MAIN_FOOTER: u32 = 0x7A66664D;
pub(crate) const FOOTER_IDENTIFIER_OBJECT_FOOTER_PHYSICAL: u32 = 0x7a666650;
pub(crate) const FOOTER_IDENTIFIER_OBJECT_FOOTER_LOGICAL: u32 = 0x7A66664C;
pub(crate) const FOOTER_IDENTIFIER_FILE_FOOTER: u32 = 0x7A666649;

pub(crate) const PBE_KDF_PARAMETERS: u32 = 0x6b646670;

// Encoding keys
pub(crate) const ENCODING_KEY_CASE_NUMBER: &str = "cn";
pub(crate) const ENCODING_KEY_EVIDENCE_NUMBER: &str = "ev";
pub(crate) const ENCODING_KEY_EXAMINER_NAME: &str = "ex";
pub(crate) const ENCODING_KEY_NOTES: &str = "no";
pub(crate) const ENCODING_KEY_ACQISITION_START: &str = "as";
pub(crate) const ENCODING_KEY_ACQISITION_END: &str = "ae";
pub(crate) const ENCODING_KEY_DESCRIPTION_NOTES: &str = "dn";

// chunk header flags
pub(crate) const ERROR_FLAG_VALUE: u8 = 1<<0;
pub(crate) const COMPRESSION_FLAG_VALUE: u8 = 1<<1;

// - Error messages
pub(crate) const ERROR_HEADER_DECODER_UNKNOWN_HASH_TYPE: &'static str = "Unknown hash type value.";
pub(crate) const ERROR_HEADER_DECODER_MISMATCH_IDENTIFIER_KDF: &'static str = "The read identifier does not match to any known KDF header identifier.";
pub(crate) const ERROR_HEADER_DECODER_UNKNOWN_PBE_SCHEME: &'static str = "Unknown PBEncryption scheme value.";
pub(crate) const ERROR_HEADER_DECODER_UNKNOWN_KDF_SCHEME: &'static str = "Unknown KDF scheme value.";
pub(crate) const ERROR_HEADER_DECODER_UNKNOWN_ENCRYPTION_ALGORITHM: &'static str = "Unknown encryption algorithm value.";
pub(crate) const FILE_EXTENSION_PARSER_ERROR: &str = "Error while trying to parse extension value";
pub(crate) const ERROR_HEADER_DECODER_HEADER_LENGTH: &'static str = "Unable to read header length from given data.";
pub(crate) const ERROR_HEADER_DECODER_KEY_POSITION: &'static str = "Key not in position.";
pub(crate) const ERROR_HEADER_DECODER_COMPRESSION_ALGORITHM: &'static str = "unknown compression algorithm value";
pub(crate) const ERROR_HEADER_DECODER_MISMATCH_IDENTIFIER: &'static str = "The read identifier does not match the header identifier.";
pub(crate) const ERROR_HEADER_DECODER_MAIN_HEADER_ENCRYPTED: &'static str = "The main header is encrypted.";
pub(crate) const ERROR_HEADER_DECODER_MAIN_HEADER_NOT_ENCRYPTED: &'static str = "The main header is not encrypted.";
pub(crate) const ERROR_MISSING_SEGMENT: &'static str = "A segment is missing.";

pub(crate) const ERROR_ZFFREADER_SEGMENT_NOT_FOUND: &'static str = "The segment of the chunk was not found.";

pub(crate) const ERROR_IO_NOT_SEEKABLE_NEGATIVE_POSITION: &'static str = "Unseekable position (position is negative).";

pub(crate) const ERROR_REWRITE_MAIN_HEADER: &'static str = "An error occurred while trying to rewrite the main header to the output file. The written data length specified is not correctly listed in the header.";

// Default values
/// the default chunk size as 2^x. This value is 15, so the default chunk size is 2^15 = 32768 bytes.
pub const DEFAULT_CHUNK_SIZE: u8 = 15;
pub(crate) const DEFAULT_LENGTH_HEADER_IDENTIFIER: usize = 4;
pub(crate) const DEFAULT_LENGTH_VALUE_HEADER_LENGTH: usize = 8;
/// The default compression ratio threshold
pub const DEFAULT_COMPRESSION_RATIO_THRESHOLD: f32 = 1.05;

/// the default key length of a public signature key
pub const ED25519_DALEK_PUBKEY_LEN: usize = PUBLIC_KEY_LENGTH;
/// the default length of an ed25519 signature
pub const ED25519_DALEK_SIGNATURE_LEN: usize = SIGNATURE_LENGTH;
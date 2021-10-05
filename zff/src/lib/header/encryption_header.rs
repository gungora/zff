// - STD
use std::io::{Cursor, Read};

// - internal
use crate::{
	Result,
	EncryptionAlgorithm,
	HeaderObject,
	HeaderEncoder,
	HeaderDecoder,
	ValueEncoder,
	ValueDecoder,
	header::PBEHeader,
	ZffError,
	KDFScheme,
	PBEScheme,
	header::KDFParameters,
	Encryption,
};

use crate::{
	HEADER_IDENTIFIER_ENCRYPTION_HEADER,
	ERROR_HEADER_DECODER_UNKNOWN_ENCRYPTION_ALGORITHM,
};

// - external
use serde::ser::{Serialize, Serializer, SerializeStruct};
use hex::ToHex;

/// The encryption header contains all informations (and the **encrypted** key) for the data and header encryption.\
/// The encryption header is the only optional header part of the main header.
#[derive(Debug,Clone)]
pub struct EncryptionHeader {
	header_version: u8,
	pbe_header: PBEHeader,
	algorithm: EncryptionAlgorithm,
	encrypted_encryption_key: Vec<u8>,
	encrypted_header_nonce: [u8; 12],
}

impl EncryptionHeader {
	/// creates a new encryption header by the given values.
	pub fn new(
		header_version: u8,
		pbe_header: PBEHeader,
		algorithm: EncryptionAlgorithm,
		encrypted_encryption_key: Vec<u8>, //encrypted with set password
		encrypted_header_nonce: [u8; 12], //used for header encryption
		) -> EncryptionHeader {
		Self {
			header_version: header_version,
			pbe_header: pbe_header,
			algorithm: algorithm,
			encrypted_encryption_key: encrypted_encryption_key,
			encrypted_header_nonce: encrypted_header_nonce
		}
	}

	/// returns the used encryption algorithm.
	pub fn algorithm(&self) -> &EncryptionAlgorithm {
		&self.algorithm
	}

	/// returns a reference to the inner PBE header.
	pub fn pbe_header(&self) -> &PBEHeader {
		&self.pbe_header
	}

	/// returns the nonce, used for header encryption
	pub fn nonce(&self) -> &[u8; 12] {
		&self.encrypted_header_nonce
	}

	/// tries to decrypt the encryption key
	pub fn decrypt_encryption_key<P: AsRef<[u8]>>(&self, password: P) -> Result<Vec<u8>> {
		match self.pbe_header.kdf_scheme() {
			KDFScheme::PBKDF2SHA256 => match self.pbe_header.kdf_parameters() {
				KDFParameters::PBKDF2SHA256Parameters(parameters) => {
					let iterations = parameters.iterations();
					let salt = parameters.salt();

					match self.pbe_header.encryption_scheme() {
						PBEScheme::AES128CBC => Encryption::decrypt_pbkdf2sha256_aes128cbc(
							iterations,
							salt,
							self.pbe_header.nonce(),
							password,
							&self.encrypted_encryption_key
							),
						PBEScheme::AES256CBC => Encryption::decrypt_pbkdf2sha256_aes256cbc(
							iterations,
							salt,
							self.pbe_header.nonce(),
							password,
							&self.encrypted_encryption_key
							),
					}
				}
				
			}
		}
	}
}

impl HeaderObject for EncryptionHeader {
	fn identifier() -> u32 {
		HEADER_IDENTIFIER_ENCRYPTION_HEADER
	}
	fn encode_header(&self) -> Vec<u8> {
		let mut vec = Vec::new();

		vec.push(self.header_version);
		vec.append(&mut self.pbe_header.encode_directly());
		vec.push(self.algorithm.clone() as u8);
		vec.append(&mut self.encrypted_encryption_key.encode_directly());
		vec.append(&mut self.encrypted_header_nonce.encode_directly());
		vec
	}
}

impl HeaderEncoder for EncryptionHeader {}

impl HeaderDecoder for EncryptionHeader {
	type Item = EncryptionHeader;

	fn decode_content(data: Vec<u8>) -> Result<EncryptionHeader> {
		let mut cursor = Cursor::new(data);
		let header_version = u8::decode_directly(&mut cursor)?;
		let pbe_header = PBEHeader::decode_directly(&mut cursor)?;
		let encryption_algorithm = match u8::decode_directly(&mut cursor)? {
			0 => EncryptionAlgorithm::AES128GCMSIV,
			1 => EncryptionAlgorithm::AES256GCMSIV,
			_ => return Err(ZffError::new_header_decode_error(ERROR_HEADER_DECODER_UNKNOWN_ENCRYPTION_ALGORITHM)),
		};
		let key_length = u32::decode_directly(&mut cursor)? as usize;
		let mut encryption_key = vec![0u8; key_length];
		cursor.read_exact(&mut encryption_key)?;
		let mut nonce = [0; 12];
		cursor.read_exact(&mut nonce)?;
		Ok(EncryptionHeader::new(header_version, pbe_header, encryption_algorithm, encryption_key, nonce))
	}
}

impl Serialize for EncryptionHeader {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("EncryptionHeader", 10)?;
        state.serialize_field("header_version", &self.header_version)?;
        state.serialize_field("pbe_header", &self.pbe_header)?;
        state.serialize_field("algorithm", &self.algorithm)?;
        state.serialize_field("encrypted_encryption_key", &self.encrypted_encryption_key.encode_hex::<String>())?;
        state.serialize_field("encrypted_header_nonce", &self.encrypted_encryption_key.encode_hex::<String>())?;
        state.end()
    }
}
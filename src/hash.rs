//! Handles the hashing of provided strings to resulting strings. Also supports hashing with byte arrays as an alternative.
//! Importing byte arrays, strings, string slices, etc is allowed.
//! Only enabled with the hash-flag set.

use sha2::{Digest, Sha256};

/// A trait indicating that a type may be input to be hashed.
pub trait IntoHashable {
    fn convert<'a>(input: &'a Self) -> Result<&'a [u8], HashError>;
}
/// A trait indicating that a type may be returned as a hash implicitly
pub trait FromHashable {
    fn restore(hashed: [u8; 32]) -> Result<Self, HashError>
    where
        Self: Sized;
}

#[derive(Debug)]
/// Represents failure states when attempting to hash a value
pub enum HashError {
    NotImplemented,
    SizeError(std::array::TryFromSliceError),
}

impl std::fmt::Display for HashError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HashError::NotImplemented => f.write_str("hashing method not implemented"),
            HashError::SizeError(_) => f.write_str(
                "Unable to hash due to internal failure (incorrect number of bytes != 32)",
            ),
        }
    }
}

/// Hashes a provided value, using sha256. This obviously should only be used with fully randomly generated input.
/// the output will be 32 bytes in size - this may be taken as a String or [u8], or [u8; 32], or any other type
/// which implements the `Hashable` trait.
pub fn hash<'a, T, G>(key: T) -> Result<G, HashError>
where
    T: IntoHashable,
    G: FromHashable,
{
    let mut hasher = Sha256::new();
    hasher.update(IntoHashable::convert(&key)?);
    let res: [u8; 32] = hasher.finalize()[..]
        .try_into()
        .map_err(HashError::SizeError)?;
    Ok(FromHashable::restore(res)?)
}

impl IntoHashable for Vec<u8> {
    fn convert<'a>(input: &'a Vec<u8>) -> Result<&'a [u8], HashError> {
        Ok(input)
    }
}

impl FromHashable for Vec<u8> {
    fn restore(hashed: [u8; 32]) -> Result<Vec<u8>, HashError> {
        Ok(hashed.into())
    }
}

impl IntoHashable for String {
    fn convert<'a>(input: &'a String) -> Result<&'a [u8], HashError> {
        Ok(input.as_bytes())
    }
}

impl FromHashable for String {
    fn restore(hashed: [u8; 32]) -> Result<String, HashError> {
        let res: String = hashed.map(|c: u8| format!("{:x}", c)).concat();
        Ok(res)
    }
}

impl IntoHashable for &[u8] {
    fn convert<'a>(input: &&'a [u8]) -> Result<&'a [u8], HashError> {
        Ok(input)
    }
}

impl IntoHashable for &str {
    fn convert<'a>(import: &&'a str) -> Result<&'a [u8], HashError> {
        Ok(import.as_bytes())
    }
}

#[test]
fn test_hash() {
    let input: Vec<u8> = "1234567890-1234567890-1234567890".as_bytes().to_vec();
    let hashed_string: String = hash(input).unwrap();
    assert_eq!(
        hashed_string,
        "f0a3f3a2377a0403f46f872bda383aab391ab987ec6ad25c27323ac16c7a3ae".to_owned()
    );
}

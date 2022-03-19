use sha2::{Digest, Sha256};

/// shelf supports a 2-layer-deep file system. When a file is uploaded, the path
/// is generated from the key following 2 steps.
///
/// First, the key is hashed using SHA-256 and the first 4 characters are used
/// to create the subdirectories: the first two for the first subdirectory name,
/// the last two for the second subdirectory name.
///
/// The key is then represented with a Base64 encoding, which gives the file name
/// as stored in the volumes.
///
/// Example:
///     key: "Hello world"
///     SHA-256: 64ec88ca00b268...
///     Base64: SGVsbG8gd29ybGQ=
///
///     The resulting path is "64/ec/SGVsbG8gd29ybGQ="
///
pub fn key_to_path<K: AsRef<[u8]>>(key: &K) -> String {
    let mut hasher = Sha256::new();
    hasher.update(&key);

    let hashed_key: String = format!("{:X}", hasher.finalize());
    let sub_dir_1 = &hashed_key[0..2];
    let sub_dir_2 = &hashed_key[2..4];
    let b64_key = base64::encode(&key);

    format!("{}/{}/{}", sub_dir_1, sub_dir_2, b64_key)
}

unsafe extern "C" {
    fn hash_one(
        header_hash_bytes: *const u8,
        nonce64_ptr: *const u64,
        block_height: i32,
        mix_out_bytes: *const u8,
        hash_out_bytes: *const u8,
    );

    fn verify(
        header_hash_bytes: *const u8,
        nonce64_ptr: *const u64,
        block_height: i32,
        mix_out_bytes: *const u8,
        hash_out_bytes: *const u8,
    ) -> bool;
}

/// Computes a KawPow hash.
///
/// # Arguments
///
/// * `header_hash` - The 32-byte header hash of the block.
/// * `nonce` - The nonce used for the hash.
/// * `block_height` - The block height at which the hash is being computed.
///
/// # Returns
///
/// A tuple `(mix_out, hash_out)`:
/// - `mix_out`: A 32-byte array representing the mix digest.
/// - `hash_out`: A 32-byte array representing the final KawPow hash.
///
/// # Safety
///
/// Internally this calls into an `unsafe` FFI function (`hash_one`).
///
/// # Examples
///
/// ```
/// use hasherkawpow_sys::hash_kawpow;
///
/// let header_hash = [0u8; 32];
/// let nonce: u64 = 42;
/// let block_height = 100;
///
/// let (mix, hash) = hash_kawpow(&header_hash, &nonce, block_height);
/// assert_eq!(mix.len(), 32);
/// assert_eq!(hash.len(), 32);
/// ```
pub fn hash_kawpow(header_hash: &[u8; 32], nonce: &u64, block_height: i32) -> ([u8; 32], [u8; 32]) {
    let mut mix_out = [0u8; 32];
    let mut hash_out = [0u8; 32];
    unsafe {
        hash_one(
            header_hash.as_ptr(),
            nonce,
            block_height,
            mix_out.as_mut_ptr(),
            hash_out.as_mut_ptr(),
        );
    }
    (mix_out, hash_out)
}

/// Verifies a KawPow hash result.
///
/// # Arguments
///
/// * `header_hash` - The 32-byte header hash of the block.
/// * `nonce` - The nonce used for the hash.
/// * `block_height` - The block height at which the hash is being verified.
/// * `mix_out` - The 32-byte mix digest that was computed.
/// * `hash_out` - The 32-byte final KawPow hash that was computed.
///
/// # Returns
///
/// `true` if the provided `mix_out` and `hash_out` are valid for the given
/// inputs, otherwise `false`.
///
/// # Safety
///
/// Internally this calls into an `unsafe` FFI function (`verify`).
///
/// # Examples
///
/// ```
/// use hasherkawpow_sys::{hash_kawpow, verify_kawpow};
///
/// let header_hash = [0u8; 32];
/// let nonce: u64 = 42;
/// let block_height = 100;
///
/// // Compute a hash
/// let (mix, hash) = hash_kawpow(&header_hash, &nonce, block_height);
///
/// // Verify the hash
/// let is_valid = verify_kawpow(&header_hash, &nonce, block_height, &mix, &hash);
/// assert!(is_valid);
/// ```
pub fn verify_kawpow(
    header_hash: &[u8; 32],
    nonce: &u64,
    block_height: i32,
    mix_out: &[u8; 32],
    hash_out: &[u8; 32],
) -> bool {
    let valid;
    unsafe {
        valid = verify(
            header_hash.as_ptr(),
            nonce,
            block_height,
            mix_out.as_ptr(),
            hash_out.as_ptr(),
        );
    }

    valid
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    // Helper to convert a hex string to Vec<u8>
    fn hex_to_bytes(hexstr: &str) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(hexstr.len() / 2);
        let chars: Vec<_> = hexstr.chars().collect();
        for i in (0..hexstr.len()).step_by(2) {
            let byte = u8::from_str_radix(&format!("{}{}", chars[i], chars[i + 1]), 16)
                .expect("valid hex");
            bytes.push(byte);
        }
        bytes
    }

    // Helper to convert big-endian hex to little-endian u64
    fn hex_to_le_u64(hexstr: &str) -> u64 {
        let mut bytes = hex_to_bytes(hexstr);
        bytes.reverse();
        let mut arr = [0u8; 8];
        arr.copy_from_slice(&bytes);
        u64::from_le_bytes(arr)
    }

    // Helper to convert Vec<u8> to hex string
    fn bytes_to_hex(bytes: &[u8]) -> String {
        bytes
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    }

    #[test]
    fn test_hash_kawpow_matches_expected() {
        let header_hash =
            hex_to_bytes("63543d3913fe56e6720c5e61e8d208d05582875822628f483279a3e8d9c9a8b3");
        let nonce = hex_to_le_u64("88a23b0033eb959b");
        let block_height = 262523i32;
        let expected_mix_hash = "89732e5ff8711c32558a308fc4b8ee77416038a70995670e3eb84cbdead2e337";
        let expected_hash = "0000000718ba5143286c46f44eee668fdf59b8eba810df21e4e2f4ec9538fc20";

        let header_hash_arr: [u8; 32] = header_hash.try_into().unwrap();
        let (mix, hash) = hash_kawpow(&header_hash_arr, &nonce, block_height);

        let mix_hex = bytes_to_hex(&mix);
        let hash_hex = bytes_to_hex(&hash);

        println!("Mix Hash: {}", mix_hex);
        println!("Expected: {}\n", expected_mix_hash);
        println!("Hash:     {}", hash_hex);
        println!("Expected: {}\n", expected_hash);

        assert_eq!(mix_hex, expected_mix_hash, "Got invalid mix hash");
        assert_eq!(hash_hex, expected_hash, "Got invalid hash");
    }

    #[test]
    fn test_verify_kawpow_matches_expected() {
        let header_hash =
            hex_to_bytes("63543d3913fe56e6720c5e61e8d208d05582875822628f483279a3e8d9c9a8b3");
        let nonce = hex_to_le_u64("88a23b0033eb959b");
        let block_height = 262523i32;
        let expected_hash = "0000000718ba5143286c46f44eee668fdf59b8eba810df21e4e2f4ec9538fc20";

        let header_hash_arr: [u8; 32] = header_hash.try_into().unwrap();
        let (mix, hash) = hash_kawpow(&header_hash_arr, &nonce, block_height);

        let valid = verify_kawpow(&header_hash_arr, &nonce, block_height, &mix, &hash);
        assert!(valid, "Verification failed");
        assert_eq!(
            bytes_to_hex(&hash),
            expected_hash,
            "Verified hash output does not match original hash"
        );
    }

    #[test]
    fn test_verify_kawpow_benchmark() {
        let header_hash =
            hex_to_bytes("63543d3913fe56e6720c5e61e8d208d05582875822628f483279a3e8d9c9a8b3");
        let nonce = hex_to_le_u64("88a23b0033eb959b");
        let block_height = 262523i32;

        let header_hash_arr: [u8; 32] = header_hash.try_into().unwrap();
        let (mix, _hash) = hash_kawpow(&header_hash_arr, &nonce, block_height);
        let hash_out_arr: [u8; 32] = [0u8; 32];

        let iterations = 1000;
        let start = Instant::now();
        for _ in 0..iterations {
            let valid = verify_kawpow(&header_hash_arr, &nonce, block_height, &mix, &hash_out_arr);
            assert!(valid, "Verification failed");
        }
        let elapsed = start.elapsed().as_millis();
        let verify_ps = (iterations as f64) / (elapsed as f64) * 1000.0;
        println!("verify/sec = {}", verify_ps);
    }
}

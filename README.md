# hasherkawpow-sys

Low-level Rust bindings for KawPow hashing and verification.

This crate provides two functions:

- `hash_kawpow` — compute a KawPow hash from a header hash, nonce, and block height.  
- `verify_kawpow` — verify a previously computed KawPow hash.  

These functions wrap FFI calls to native implementations via `unsafe` code,  
but expose a safe Rust API with fixed-size arrays.

---

## Upstream Source

The underlying C source code is from  
[MintPond's hasher-kawpow library](https://github.com/MintPond/hasher-kawpow),  
which itself adapts most of its native code from the  
[Ravencoin project](https://github.com/RavenProject/Ravencoin).

---

## Example

```rust
use hasherkawpow_sys::{hash_kawpow, verify_kawpow};

let header_hash = [0u8; 32];
let nonce: u64 = 42;
let block_height = 100;

// Compute a hash
let (mix, hash) = hash_kawpow(&header_hash, &nonce, block_height);

// Verify the hash
assert!(verify_kawpow(&header_hash, &nonce, block_height, &mix, &hash));
```
---
## Testing

Run tests with `cargo test -- --test-threads=1`. `cargo test` will occasionally fail at `src/ethash/progpow.cpp:190`

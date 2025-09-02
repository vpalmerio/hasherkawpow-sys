unsafe extern "C" {
    
    fn hash_one (
        header_hash_bytes: *const u8,
        nonce64_ptr: *const u64,
        block_height: u64,
        mix_out_bytes: *const u8,
        hash_out_bytes: *const u8,
    );

    fn verify (
        header_hash_bytes: *const u8,
        nonce64_ptr: *const u64,
        block_height: u64,
        mix_out_bytes: *const u8,
        hash_out_bytes: *const u8,
    ) -> bool; 
}

pub fn hash_kawpow(header_hash: Vec<u8>, nonce: &u64, block_height: u64) -> (Vec<u8>, Vec<u8>)
{
        //with help from:
        //https://users.rust-lang.org/t/how-to-return-byte-array-from-rust-function-to-ffi-c/18136
        //https://users.rust-lang.org/t/deallocating-box-from-raw-mut-u8/61444
        //https://doc.rust-lang.org/std/mem/fn.forget.html

        let mut header_hash_buf  = 
            std::mem::ManuallyDrop::new(header_hash);
        let header_hash_buf_ptr = header_hash_buf.as_mut_ptr();

        let mut mix_out_hash_buf  = 
            std::mem::ManuallyDrop::new(vec![0; 32]);
        let mix_out_hash_buf_ptr = mix_out_hash_buf.as_mut_ptr();
        
        let mut hash_out_buf  = 
            std::mem::ManuallyDrop::new(vec![0; 32]);
        let hash_out_buf_ptr = hash_out_buf.as_mut_ptr();

        unsafe {
            hash_one (
                header_hash_buf_ptr,
                nonce,
                block_height,
                mix_out_hash_buf_ptr,
                hash_out_buf_ptr,
            ); 
        }
        
        //reclaim memory/buffer
        let mix_out_vec_reclaimed: Vec<u8> = unsafe {
            Vec::from_raw_parts(mix_out_hash_buf_ptr, 32, 32)
        };

        let hash_out_vec_reclaimed: Vec<u8> = unsafe {
            Vec::from_raw_parts(hash_out_buf_ptr, 32, 32)
        };

        let _header_hash_vec_reclaimed: Vec<u8> = unsafe {
            Vec::from_raw_parts(header_hash_buf_ptr, 32, 32)
        };

        return (mix_out_vec_reclaimed, hash_out_vec_reclaimed);

    
}

pub fn verify_kawpow(
    header_hash: Vec<u8>, 
    nonce: &u64, 
    block_height: u64, 
    mix_out_vec: Vec<u8>, 
    hash_out_vec: Vec<u8>
) -> bool
{
        //with help from:
        //https://users.rust-lang.org/t/how-to-return-byte-array-from-rust-function-to-ffi-c/18136
        //https://users.rust-lang.org/t/deallocating-box-from-raw-mut-u8/61444
        //https://doc.rust-lang.org/std/mem/fn.forget.html
        

        let mut header_hash_buf  = 
            std::mem::ManuallyDrop::new(header_hash);
        let header_hash_buf_ptr = header_hash_buf.as_mut_ptr();

        let mut mix_out_hash_buf  = 
            std::mem::ManuallyDrop::new(mix_out_vec);
        let mix_out_hash_buf_ptr = mix_out_hash_buf.as_mut_ptr();
        
        let mut hash_out_buf  = std::mem::ManuallyDrop::new(hash_out_vec);
        let hash_out_buf_ptr = hash_out_buf.as_mut_ptr();

        let valid;

        unsafe {
            valid = verify (
                header_hash_buf_ptr,
                nonce,
                block_height,
                mix_out_hash_buf_ptr,
                hash_out_buf_ptr,
            ); 
        }
        
        //reclaim memory/buffer to properly free it
        let _mix_out_vec_reclaimed: Vec<u8> = unsafe {
            Vec::from_raw_parts(mix_out_hash_buf_ptr, 32, 32)
        };

        let _hash_out_vec_reclaimed: Vec<u8> = unsafe {
            Vec::from_raw_parts(hash_out_buf_ptr, 32, 32)
        };

        let _header_hash_vec_reclaimed: Vec<u8> = unsafe {
            Vec::from_raw_parts(header_hash_buf_ptr, 32, 32)
        };

        return valid;
}

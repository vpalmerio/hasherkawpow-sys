#include <stdint.h>
#include <iostream>

#include "include/ethash.h"
#include "include/ethash.hpp"
#include "include/progpow.hpp"
#include "uint256.h"
#include "helpers.hpp"

extern "C" void hash_one (
    const uint8_t *header_hash_bytes, 
    const uint64_t* nonce64_ptr, 
    int block_height,
    const uint8_t *mix_out_bytes, 
    const uint8_t *hash_out_bytes 
) {

    const ethash::hash256* header_hash_ptr = (ethash::hash256*)header_hash_bytes;
    ethash::hash256* mix_out_ptr = (ethash::hash256*)mix_out_bytes;
    ethash::hash256* hash_out_ptr = (ethash::hash256*)hash_out_bytes;

    static ethash::epoch_context_ptr context{nullptr, nullptr};
    
    const auto epoch_number = ethash::get_epoch_number(block_height);

    if (!context || context->epoch_number != epoch_number)
        context = ethash::create_epoch_context(epoch_number);

    return progpow::hash_one(*context, block_height, header_hash_ptr, *nonce64_ptr, mix_out_ptr, hash_out_ptr);
}

extern "C" bool verify (
    const uint8_t *header_hash_bytes, 
    const uint64_t* nonce64_ptr, 
    int block_height,
    uint8_t *mix_out_bytes, 
    uint8_t *hash_out_bytes 
) {
    const ethash::hash256* header_hash_ptr = (ethash::hash256*)header_hash_bytes;
    const ethash::hash256* mix_hash_ptr = (ethash::hash256*)mix_out_bytes;
    ethash::hash256* hash_out_ptr = (ethash::hash256*)hash_out_bytes;

    static ethash::epoch_context_ptr context{nullptr, nullptr};

    const auto epoch_number = ethash::get_epoch_number(block_height);

    if (!context || context->epoch_number != epoch_number)
        context = ethash::create_epoch_context(epoch_number);

    return progpow::verify(*context, block_height, header_hash_ptr, *mix_hash_ptr, *nonce64_ptr, hash_out_ptr);
}

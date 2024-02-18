//extern crate rand;
use rand;

use std::collections::HashSet;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

fn main() {
    // Example k-mer sets (replace these with your actual k-mer sets)
    let set1: HashSet<&str> = ["AAAA", "AAAT", "AAAG", "AAAC"].iter().cloned().collect();
    let set2: HashSet<&str> = ["AAAT", "AAAG", "AAAC", "AATT"].iter().cloned().collect();

    // Define parameters
    let num_hashes = 100; // Number of hash functions (controls accuracy vs speed trade-off)
    let seed: [u8; 32] = [1; 32]; // Seed for random number generator

    // Initialize MinHash signatures
    let mut sig1 = vec![u64::max_value(); num_hashes];
    let mut sig2 = vec![u64::max_value(); num_hashes];

    // Initialize random number generator
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    // Generate hash functions (random permutations of 0..num_hashes)
    let hash_functions: Vec<_> = (0..num_hashes).collect();

    // Update MinHash signatures
    for kmer in set1.union(&set2) {
        let mut hasher = rng.clone();
        for (i, &hash_fn) in hash_functions.iter().enumerate() {
            let hash_val = hash(kmer, hash_fn);
            if set1.contains(kmer) {
                sig1[i] = std::cmp::min(sig1[i], hash_val);
            }
            if set2.contains(kmer) {
                sig2[i] = std::cmp::min(sig2[i], hash_val);
            }
        }
    }

    // Calculate Jaccard similarity approximation
    let mut shared = 0;
    let mut total = 0;
    for (hash1, hash2) in sig1.iter().zip(sig2.iter()) {
        if hash1 == hash2 {
            shared += 1;
        }
        total += 1;
    }
    let similarity = shared as f64 / total as f64;

    println!("Estimated Jaccard similarity: {}", similarity);
}

// Hash function (murmurhash3)
fn hash(kmer: &str, seed: usize) -> u64 {
    let mut h = murmurhash3_x64_128(kmer.as_bytes(), seed as u32);
    h ^= h >> 33;
    h.wrapping_mul(0xff51afd7ed558ccd)
}

// MurmurHash3 implementation
fn murmurhash3_x64_128(data: &[u8], seed: u32) -> u64 {
    const C1: u64 = 0x87c37b91114253d5;
    const C2: u64 = 0x4cf5ad432745937f;
    const R1: u32 = 31;
    const R2: u32 = 27;
    const M: u64 = 0x5bd1e995;
    const N: u32 = 0x100000000;
    let mut h1 = seed as u64;
    let mut h2 = seed as u64;
    let mut data = data;
    while data.len() >= 16 {
        let k1 = u64::from_le_bytes([data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]]);
        let k2 = u64::from_le_bytes([data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15]]);
        data = &data[16..];
        h1 ^= mix_k1(k1);
        h1 = rotl64(h1, R1);
        h1 = h1.wrapping_add(h2);
        h1 = h1.wrapping_mul(M);
        h2 ^= mix_k2(k2);
        h2 = rotl64(h2, R2);
        h2 = h2.wrapping_add(h1);
        h2 = h2.wrapping_mul(M);
    }
    if data.len() > 0 {
        let k1 = remaining(data);
        h1 ^= mix_k1(k1);
        h1 = rotl64(h1, R1);
        h1 = h1.wrapping_add(h2);
        h1 = h1.wrapping_mul(M);
    }
    if data.len() > 8 {
        let k2 = remaining(&data[8..]);
        h2 ^= mix_k2(k2);
        h2 = rotl64(h2, R2);
        h2 = h2.wrapping_add(h1);
        h2 = h2.wrapping_mul(M);
    }
    h1 ^= (data.len() as u64);
    h2 ^= (data.len() as u64);
    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);
    h1 = fmix64(h1);
    h2 = fmix64(h2);
    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);
    h1
}

fn rotl64(x: u64, r: u32) -> u64 {
    (x << r) | (x >> (64 - r))
}

fn mix_k1(k1: u64) -> u64 {
    k1.wrapping_mul(C1).rotate_left(31).wrapping_mul(C2)
}

fn mix_k2(k2: u64) -> u64 {
    k2.wrapping_mul(C2).rotate_left(33).wrapping_mul(C1)
}

fn remaining(data: &[u8]) -> u64 {
    let mut key: u64 = 0;
    let mut i = 0;
    let mut shift = 0;
    while i < data.len() {
        key ^= u64::from(data[i]) << shift;
        shift += 8;
        i += 1;
    }
    key
}

fn fmix64(mut k: u64) -> u64 {
    k ^= k >> 33;
    k = k.wrapping_mul(0xff51afd7ed558ccd);
    k ^= k >> 33;
    k = k.wrapping_mul(0xc4ceb9fe1a85ec53);
    k ^= k >> 33;
    k
}

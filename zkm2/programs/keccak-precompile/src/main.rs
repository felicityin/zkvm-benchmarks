#![no_main]

extern crate alloc;

zkm_zkvm::entrypoint!(main);
use zkm_zkvm::lib::keccak256::keccak256;
// use zkm_zkvm::lib::hasher::Hasher;
pub fn main() {
    let input: Vec<u8> = zkm_zkvm::io::read();
    let result = keccak256(&input.as_slice());
    zkm_zkvm::io::commit::<[u8; 32]>(&result.into());
}

// fn keccak256<T: AsRef<[u8]>>(bytes: T) -> [u8; 32] {
//     let mut output = [0u8; 32];
//     let mut hasher = zkm_zkvm::lib::keccak::Keccak::v256();
//     hasher.update(bytes.as_ref());
//     hasher.finalize(&mut output);
//     output
// }
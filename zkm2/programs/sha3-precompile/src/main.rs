#![no_main]

extern crate alloc;

zkm_zkvm::entrypoint!(main);
use zkm_zkvm::lib::sha3::sha3_256;
pub fn main() {
    let input: Vec<u8> = zkm_zkvm::io::read();
    let result = sha3_256(&input.as_slice());
    zkm_zkvm::io::commit::<[u8; 32]>(&result.into());
}

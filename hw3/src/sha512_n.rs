use sha2::{Sha256, Sha512, Digest};

pub struct sha512_n {
    n : usize, 
}

impl  sha512_n {
    pub fn hash_func(&self, input: &str) -> String {
        let digest = String::from_utf8(Sha512::digest(input).to_vec()).unwrap();
        digest[..self.n].to_string()
    }
}
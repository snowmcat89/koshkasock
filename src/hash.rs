
use sha2::{Digest, Sha256};


pub fn hash_256_(passwd : &str)-> String{
    let mut hasha = Sha256::new();
    hasha.update(passwd);
    format!("{:x}", hasha.finalize())
}

// u see this ??? 




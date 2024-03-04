use crypto::sha1::Sha1;
use crypto::digest::Digest;

pub fn hash(data: &String) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(data);
    hasher.result_str()
}
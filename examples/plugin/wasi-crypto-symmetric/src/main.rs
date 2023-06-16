use wasi_crypto_symmetric::encryption::{ symmetric_key_generate};

fn main() {
    let encryption_algo = "SHA-256";
    // let text = "This is the text to be encrypted";

    let key = symmetric_key_generate::<Vec<u8>>(encryption_algo);

    println!("{:?}", key);
}

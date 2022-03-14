use std::{io, str};
use x25519_dalek::{PublicKey, StaticSecret};

fn main() {
    let bob_hex = include_str!("../bob.hex");
    let mut bob_buf = [0u8; 32];
    hex::decode_to_slice(bob_hex, &mut bob_buf).expect("hex to bytes");
    let bob = StaticSecret::from(bob_buf);

    println!("Enter id :");

    let mut input = String::new();
    let mut buf = [0u8; 32];

    loop {
        io::stdin().read_line(&mut input).unwrap();

        if input.len() == 65 { break }
        else {
            println!("Wrong id size, must be 64 characters (found {}) :", input.len()-1);
            input.clear();
        }
    }

    hex::decode_to_slice(input.trim(), &mut buf)
        .expect("Wrong id format, must be hexadecimal");

    let victim = PublicKey::from(buf);

    let key = bob.diffie_hellman(&victim);

    let mut buf = [0u8; 64];
    hex::encode_to_slice(key.as_bytes(), &mut buf).expect("key to hex");

    println!("Here is the key :\n{}", str::from_utf8(&buf).unwrap());
}

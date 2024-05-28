use std::io;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

fn main() {
    let mut buffer = String::with_capacity(200);
    println!("Please enter password: \n");
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).expect("No input!");
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2.hash_password(&buffer.trim().as_bytes(), &salt)
        .expect("Password hashing has gone wrong").to_string();
    let hash = PasswordHash::new(&password_hash)
        .expect("Hashing has gone wrong")
        .hash.unwrap().to_string();
    println!("salt: {}\nhash: {}", &salt.to_string(), &hash);
}

use anyhow::{Result, anyhow};
// Import relevant traits
use chacha20poly1305::{
    ChaCha20Poly1305, Nonce,
    Key,
    aead::{AeadCore, AeadInPlace, KeyInit, OsRng },
};

pub fn u32_to_array(value:u32,array:&mut [u8])-> Result<()>{
    assert!(array.len() >= 4);
    array[0] =  (value & 0xff) as u8;
    array[1] =  (value & (0xff) << 8) as u8;
    array[2] =  (value & (0xff) << 16) as u8 ;
    array[3] =  (value & (0xff) << 24) as u8;
    Ok(())
}

pub fn encrypt_plaintext(text: &mut Vec<u8>,key:&Key,counter:&mut u32) -> Result<()> {
    println!("counter:{}",counter);
    let cipher = ChaCha20Poly1305::new(&key);
   // let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let mut nonce :Nonce = [0;12].into();
    u32_to_array(*counter,&mut nonce);
    match cipher.encrypt_in_place(&nonce, b"", text) { // TODO look into associated data
    Ok(_) =>{
        *counter+=1;
        return Ok(())},
    Err(err) =>return Err(anyhow!(err)),

    }
}

pub fn decrypt_ciphertext(text: &mut Vec<u8>,key : &Key, counter:&mut u32) -> Result<()> {
    let cipher = ChaCha20Poly1305::new(&key);
   // let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let mut nonce :Nonce = [0;12].into();
    u32_to_array(*counter,&mut nonce);
    match cipher.decrypt_in_place(&nonce, b"", text) { // TODO look into associated data
    Ok(_) =>{
        *counter+=1;
        return Ok(())},
    Err(err) =>return Err(anyhow!(err)),

    }
}

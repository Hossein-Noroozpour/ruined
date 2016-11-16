extern crate crypto;
extern crate rand;

use crypto::{
    symmetriccipher,
    buffer,
    aes,
    blockmodes
};
use crypto::buffer::{
    ReadBuffer,
    WriteBuffer,
    BufferResult
};

pub enum Error {
    Length,
    Padding,
}

pub fn en(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, Error> {
    let mut encryptor = aes::cbc_encryptor(
        aes::KeySize::KeySize256,
        key,
        iv,
        blockmodes::PkcsPadding);
    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);
    loop {
        let result = match encryptor.encrypt(&mut read_buffer, &mut write_buffer, true) {
            Ok(r) => r,
            Err(e) => {
                match e {
                    symmetriccipher::SymmetricCipherError::InvalidLength =>
                        return Err(Error::Length),
                    symmetriccipher::SymmetricCipherError::InvalidPadding =>
                        return Err(Error::Padding),
                }
            },
        };
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }
    return Ok(final_result);
}

fn de(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, Error> {
    let mut decryptor = aes::cbc_decryptor(
        aes::KeySize::KeySize256,
        key,
        iv,
        blockmodes::PkcsPadding);
    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);
    loop {
        let result = match matchdecryptor.decrypt(&mut read_buffer, &mut write_buffer, true) {
            Ok(r) => r,
            Err(e) => {
                match e {
                    symmetriccipher::SymmetricCipherError::InvalidLength =>
                        return Err(Error::Length),
                    symmetriccipher::SymmetricCipherError::InvalidPadding =>
                        return Err(Error::Padding),
                }
            },
        };
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }
    return Ok(final_result);
}
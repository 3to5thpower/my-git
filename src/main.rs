use crate::object::blob;
use crate::object::GitObject;
use libflate::zlib::{Decoder, Encoder};
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub mod object;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let sub_cmd = args.get(1).unwrap().clone();

    match sub_cmd.as_str() {
        "cat-file" => {
            let obj = cat_file_p(args.get(2).unwrap().clone())?;
            println!("{}", obj);
            Ok(())
        }
        "hash-object" => {
            let blob = hash_object(args.get(2).unwrap().clone())?;
            println!("{}", hex::encode(blob.calc_hash()));
            Ok(())
        }
        _ => Ok(()),
    }
}

pub fn cat_file_p(hash: String) -> io::Result<GitObject> {
    let (sub_dir, file) = hash.split_at(2);
    let path = format!(".git/objects/{}/{}", sub_dir, file);

    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let mut decoder = Decoder::new(buf.as_slice())?;
    let mut buf = Vec::new();
    decoder.read_to_end(&mut buf)?;

    GitObject::new(&buf).ok_or(io::Error::from(io::ErrorKind::InvalidData))
}

pub fn hash_object(path: String) -> io::Result<blob::Blob> {
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    blob::Blob::from(&buf).ok_or(io::Error::from(io::ErrorKind::InvalidInput))
}

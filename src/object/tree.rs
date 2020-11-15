use super::ObjectType;
#[cfg(feature = "json")]
use serde::Serialize;
use sha1::{Digest, Sha1};
use std::fmt;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize))]
pub struct Tree {
    pub contents: Vec<File>,
}
impl Tree {
    pub fn from(bytes: &[u8]) -> Option<Self> {
        let contents: Vec<File> = Vec::new();
        let mut iter = bytes.split(|&b| b == b'\0');

        let mut header = iter().next()?;
        let contents = iter.try_fold(contents, |mut acc, x| {
            let (hash, next_header) = x.split_at(20); //hash値は20byteなので20個で分割する
            let file = File::from(header, hash)?;

            acc.push(file);
            header = next_header;
            Some(acc)
        })
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let content: Vec<u8> = self.contents.iter().flat_map(|file| file.encode()).collect();
        let header = format!("tree {}\0", content.len());

        [header.as_bytes(), content.as_slice()].concat()
    }
}

pub struct File {
    pub mode: usize,
    pub name: String,
    pub hash: Vec<u8>,
}
impl File {
    pub fn from(header: &[u8], hash: &[u8]) -> Option<Self> {
        let split_header = String::from_utf8(header.to_vec()).ok()?;

        let mut iter = split_header.split_whitespace();

        let mode = iter.next().and_then(|word| word.parse::<usize>().ok())?;
        let name = iter.next()?;
        Some(Self::new(mode, String::from(name), hash))
    }
    pub fn encode(&self) -> Vec<u8> {
        let header = format!("{} {}\0", self.mode, self.name);
        [header.as_bytes(), &self.hash].concat()
    }
}
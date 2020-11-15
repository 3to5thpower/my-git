pub mod blob;
pub mod commit;
pub mod tree;

use blob::Blob;
use commit::Commit;
#[cfg(feature = "json")]
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::fmt;
use tree::Tree;

#[derive(Debug, Clone)]
pub enum ObjectType {
    Blob,
    Tree,
    Commit,
}

impl ObjectType {
    pub fn from(s: &str) -> Option<Self> {
        let mut header = s.split_whitespace();
        match header.next()? {
            "blob" => Some(ObjectType::Blob),
            "tree" => Some(ObjectType::Tree),
            "commit" => Some(ObjectType::Commit),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match *self {
            ObjectType::Blob => String::from("blob"),
            ObjectType::Tree => String::from("tree"),
            ObjectType::Commit => String::from("commit"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum GitObject {
    Blob(Blob),
    Tree(Tree),
    Commit(Commit),
}

impl GitObject {
    pub fn calc_hash(&self) -> Vec<u8> {
        match self {
            Self::Blob(obj) => obj.calc_hash(),
            Self::Tree(obj) => obj.calc_hash(),
            Self::Commit(obj) => obj.calc_hash(),
        }
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            Self::Blob(obj) => obj.as_bytes(),
            Self::Tree(obj) => obj.as_bytes(),
            Self::Commit(obj) => obj.as_bytes(),
        }
    }

    pub fn new(bytes: &[u8]) -> Option<Self> {
        let iter = bytes.splitn(2, |&byte| byte == b'\0');

        let obj_type = iter
            .next()
            .and_then(|x| String::from_utf8(x.to_vec()).ok())
            .and_then(|x| ObjectType::from(&x))?;

        iter.next().and_then(|x| match obj_type {
            ObjectType::Blob => Blob::from(x).map(GitObject::Blob),
            ObjectType::Tree => Tree::from(x).map(GitObject::Tree),
            ObjectType::Commit => Commit::from(x).map(GitObject::Commit),
        })
    }
}

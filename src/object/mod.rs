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
pub enum GitObject {
    Blob(Blob),
    Tree(Tree),
    Commit(Commit),
}

//! Collections
//! 
//! Only available when `collection`-feature is active as they rely on third-party Rust crates.
//! 
//! Available collections:
//! * Hashtable: [hashbrown](https://github.com/rust-lang/hashbrown)

extern crate hashbrown;

pub use hashbrown::{
    hash_map, hash_set, hash_table, DefaultHashBuilder, Equivalent, HashMap, HashSet, HashTable,
    TryReserveError,
};

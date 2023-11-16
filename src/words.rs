use serde::{Serialize , Deserialize};
use std::collections::HashMap;
use std::vec::Vec;
pub enum Words {
    Verb,
    Adj,
}

pub struct Base {
    dic : Option<String>,
    imi : Option<String>,
    sentence : Vector<String>,
    sup : Option<String>,
    hannka : HashMap<String>,
    count : i32,
}

pub struct Verb : Base {
    Base : base,
}

pub struct Adj : Base {
    Base : base,
}

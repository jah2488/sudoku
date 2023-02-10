use std::{
    collections::HashSet,
    fmt::{self, Debug},
};

use bevy::reflect::{FromReflect, Reflect};
use Value::*;

#[derive(Reflect, Clone, Eq, PartialEq, Hash, Copy, Debug, PartialOrd, Ord, Default)]
pub enum Value {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    #[default]
    Unknown,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", from_val(*self))
    }
}

impl FromReflect for Value {
    fn from_reflect(v: &dyn Reflect) -> Option<Value> {
        match v.downcast_ref::<Value>() {
            Some(v) => Some(*v),
            None => None,
        }
    }
}

pub fn options() -> HashSet<Value> {
    HashSet::from([One, Two, Three, Four, Five, Six, Seven, Eight, Nine])
}

pub fn to_val(n: u8) -> Value {
    match n {
        1 => One,
        2 => Two,
        3 => Three,
        4 => Four,
        5 => Five,
        6 => Six,
        7 => Seven,
        8 => Eight,
        9 => Nine,
        _ => Unknown,
    }
}

pub fn from_val(v: Value) -> u8 {
    match v {
        One => 1,
        Two => 2,
        Three => 3,
        Four => 4,
        Five => 5,
        Six => 6,
        Seven => 7,
        Eight => 8,
        Nine => 9,
        Unknown => 0,
    }
}

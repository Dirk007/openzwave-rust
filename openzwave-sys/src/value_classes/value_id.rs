use std::convert::{TryFrom};
use std::fmt;
#[cfg(feature = "serde_serialization")]
use serde::{Serialize, Deserialize};

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ValueGenre {
    Basic = 0,
    User = 1,
    Config = 2,
    System = 3,
    Count = 4,
}

impl TryFrom<u8> for ValueGenre {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Basic),
            1 => Ok(Self::User),
            2 => Ok(Self::Config),
            3 => Ok(Self::System),
            4 => Ok(Self::Count),
            _ => Err(()),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ValueType {
    Bool = 0,
    Byte,
    Decimal,
    Int,
    List, //< ?
    Schedule, //< ?
    Short,
    String,
    Button,
    Raw,
    //
    Unknown = 255,
    //ValueType_Max = ValueType_Raw // likely useless in Rust wrapper
}

impl TryFrom<u8> for ValueType {
    type Error = ();
    fn try_from(value: u8) -> Result<ValueType, Self::Error> {
        match value {
            0 => Ok(Self::Bool),
            1 => Ok(Self::Byte),
            2 => Ok(Self::Decimal),
            3 => Ok(Self::Int),
            4 => Ok(Self::List),
            5 => Ok(Self::Schedule),
            6 => Ok(Self::Short),
            7 => Ok(Self::String),
            8 => Ok(Self::Button),
            9 => Ok(Self::Raw),
            _ => Err(()),
        }
    }
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// IMPORTANT: This ValueID struct MUST match the layout of the OpenZWave
//            ValueID class.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde_serialization", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ValueID {
    pub id: u32,
    pub id1: u32,
    pub home_id: u32,
}

use std::convert::{TryFrom};
use std::fmt;

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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueID {
    pub id: u32,
    pub id1: u32,
    pub home_id: u32,
}

extern "C" {
    pub fn value_id_from_packed_id(home_id: u32, id: u64) -> ValueID;
    pub fn value_id_from_values(
        home_id: u32,
        node_id: u8,
        genre: ValueGenre,
        command_class_id: u8,
        instance: u8,
        value_index: u8,
        value_type: ValueType,
    ) -> ValueID;

    pub fn value_id_get_home_id(value: *const ValueID) -> u32;
    pub fn value_id_get_node_id(value: *const ValueID) -> u8;
    pub fn value_id_get_genre(value: *const ValueID) -> ValueGenre;
    pub fn value_id_get_command_class_id(value: *const ValueID) -> u8;
    pub fn value_id_get_instance(value: *const ValueID) -> u8;
    pub fn value_id_get_index(value: *const ValueID) -> u8;
    pub fn value_id_get_type(value: *const ValueID) -> ValueType;
    pub fn value_id_get_id(value: *const ValueID) -> u64;

    // Comparison Operators
    pub fn value_id_eq(myself: *const ValueID, other: *const ValueID) -> bool;
    pub fn value_id_less_than(myself: *const ValueID, other: *const ValueID) -> bool;
}

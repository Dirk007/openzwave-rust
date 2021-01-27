use crate::error::{Error, GetSetError, Result};
use ffi::manager as extern_manager;
use ffi::utils::res_to_result;
use ffi::value_classes::value_id as extern_value_id;
use libc::{c_char, c_void};
use std::ffi::CString;
use std::fmt;
use std::ptr;
use std::convert::{TryInto};

pub use ffi::value_classes::value_id::{ValueGenre, ValueType};

// Helper to have a hashable float
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct DecimalValue {
    pub value: i64,
    pub precision: u8,
}

// Rustified ValueType
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum ValueContent {
    Bool(bool),
    Byte(u8),
    Decimal(DecimalValue),
    Int(i32),
    List(String), //< ?
    Schedule, //< ?
    Short(i16),
    String(String),
    Button(bool),
    Raw, //< ? Vec<u8>?
    //
    Unknown, //< null
}

impl ToString for DecimalValue {
    fn to_string(&self) -> String {
        let shift = 10_f64.powi(self.precision as i32);
        let value: f64 = self.value as f64 / shift;
        format!("{:.1$}", value, self.precision as usize)
    }
}

impl DecimalValue {
    pub fn from_f32(value: f32, precision: u8) -> Self {
        let shift = 10_f32.powi(precision as i32);
        Self {
            value: (value * shift).trunc() as i64,
            precision
        }
    }
}

impl fmt::Display for ValueContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {            
            Self::Bool(val) | 
            Self::Button(val) => write!(f, "{}", val),
            Self::Byte(val) => write!(f, "{}", val),
            Self::Int(val) => write!(f, "{}", val),
            Self::Short(val) => write!(f, "{}", val),
            Self::String(val) | Self::List(val) => write!(f, "\"{}\"", val),
            Self::Decimal(val) => write!(f, "{}", val.to_string()),
            Self::Unknown | Self::Schedule | Self::Raw => write!(f, "null"),
        }
    }
}

// Mapping comes from https://github.com/OpenZWave/open-zwave-control-panel/blob/master/zwavelib.cpp
c_like_enum! {
    CommandClass {
        NoOperation = 0,
        Basic = 0x20,
        ControllerReplication = 0x21,
        ApplicationStatus = 0x22,
        ZipServices = 0x23,
        ZipServer = 0x24,
        SwitchBinary = 0x25,
        SwitchMultilevel = 0x26,
        SwitchAll = 0x27,
        SwitchToggleBinary = 0x28,
        SwitchToggleMultilevel = 0x29,
        ChimneyFan = 0x2A,
        SceneActivation = 0x2B,
        SceneActuatorConf = 0x2C,
        SceneControllerConf = 0x2D,
        ZipClient = 0x2E,
        ZipAdvServices = 0x2F,
        SensorBinary = 0x30,
        SensorMultilevel = 0x31,
        Meter = 0x32,
        Color = 0x33,
        ZipAdvClient = 0x34,
        MeterPulse = 0x35,
        ThermostatHeating = 0x38,
        ThermostatMode = 0x40,
        ThermostatOperatingState = 0x42,
        ThermostatSetpoint = 0x43,
        ThermostatFanMode = 0x44,
        ThermostatFanState = 0x45,
        ClimateControlSchedule = 0x46,
        ThermostatSetback = 0x47,
        DoorLockLogging = 0x4C,
        ScheduleEntryLock = 0x4E,
        BasicWindowCovering = 0x50,
        MtpWindowCovering = 0x51,
        Crc16Encap = 0x56,
        DeviceResetLocally = 0x5A,
        CentralScene = 0x5B,
        ZWavePlusInfo = 0x5E,
        MultiInstance = 0x60,
        DoorLock = 0x62,
        UserCode = 0x63,
        Configuration = 0x70,
        Alarm = 0x71,
        ManufacturerSpecific = 0x72,
        Powerlevel = 0x73,
        Protection = 0x75,
        Lock = 0x76,
        NodeNaming = 0x77,
        FirmwareUpdateMd = 0x7A,
        GroupingNane = 0x7B,
        RemoteAssociationActivate = 0x7C,
        RemoteAssociation = 0x7D,
        Battery = 0x80,
        Clock = 0x81,
        Hail = 0x82,
        WakeUp = 0x84,
        Association = 0x85,
        Version = 0x86,
        Indicator = 0x87,
        Proprietary = 0x88,
        Language = 0x89,
        Time = 0x8A,
        TimeParameters = 0x8B,
        GeographicLocation = 0x8C,
        Composite = 0x8D,
        MultiInstanceAssociation = 0x8E,
        MultiCmd = 0x8F,
        EnergyProduction = 0x90,
        ManufacturerProprietary = 0x91,
        ScreenMd = 0x92,
        ScreenAttributes = 0x93,
        SimpleAvControl = 0x94,
        AvContentDirectoryMd = 0x95,
        AvRendererStatus = 0x96,
        AvContentSearchMd = 0x97,
        Security = 0x98,
        AvTaggingMd = 0x99,
        IpConfiguration = 0x9A,
        AssociationCommandConfiguration = 0x9B,
        SensorAlarm = 0x9C,
        SilenceAlarm = 0x9D,
        SensorConfiguration = 0x9E,
        Mark = 0x9F,
        NonInteroperable = 0xF0
    }
}

impl fmt::Display for CommandClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

use crate::controller::Controller;
use ffi::utils::{
    recover_string, recover_vec, rust_string_creator, rust_string_vec_creator, rust_vec_creator,
};
use crate::node::Node;

pub struct ValueList {
    id: extern_value_id::ValueID,
}

impl ValueList {
    pub fn selection_as_string(&self) -> Result<String> {
        let manager_ptr = unsafe { extern_manager::get() };
        let mut raw_string: *mut c_char = ptr::null_mut();

        let res = unsafe {
            extern_manager::get_value_list_selection_as_string(
                manager_ptr,
                &self.id,
                &mut raw_string,
                rust_string_creator,
            )
        };

        if res {
            Ok(recover_string(raw_string))
        } else {
            Err(Error::GetError(GetSetError::APIError(
                "ValueList::selection_as_string",
            )))
        }
    }

    pub fn selection_as_int(&self) -> Result<i32> {
        let manager_ptr = unsafe { extern_manager::get() };
        let mut val: i32 = 0;
        let res = unsafe {
            extern_manager::get_value_list_selection_as_int(
                manager_ptr,
                &self.id,
                &mut val,
            )
        };
        if res {
            Ok(val)
        } else {
            Err(Error::GetError(GetSetError::APIError(
                "ValueList::selection_as_int",
            )))
        }
    }

    pub fn items(&self) -> Result<Box<Vec<String>>> {
        let manager_ptr = unsafe { extern_manager::get() };
        let mut c_items: *mut Vec<String> = ptr::null_mut();
        let c_items_void_ptr = &mut c_items as *mut *mut _ as *mut *mut c_void;
        let res = unsafe {
            extern_manager::get_value_list_items(
                manager_ptr,
                &self.id,
                c_items_void_ptr,
                rust_string_vec_creator,
            )
        };
        if res {
            Ok(recover_vec(c_items))
        } else {
            Err(Error::GetError(GetSetError::APIError("ValueList::items")))
        }
    }

    pub fn values(&self) -> Result<Box<Vec<i32>>> {
        let manager_ptr = unsafe { extern_manager::get() };
        let mut c_values: *mut Vec<i32> = ptr::null_mut();
        let c_values_void_ptr = &mut c_values as *mut *mut _ as *mut *mut c_void;
        let res = unsafe {
            extern_manager::get_value_list_values(
                manager_ptr,
                &self.id,
                c_values_void_ptr,
                rust_vec_creator::<i32>,
            )
        };
        if res {
            Ok(recover_vec(c_values))
        } else {
            Err(Error::GetError(GetSetError::APIError("ValueList::values")))
        }
    }
}

impl fmt::Debug for ValueList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ValueList {{ selection_as_string: {:?}, selection_as_int: {:?}, items: {:?}, values: {:?} }}",
               self.selection_as_string().ok(),
               self.selection_as_int().ok(),
               self.items().ok(),
               self.values().ok()
        )
    }
}

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct ValueID {
    home_id: u32,
    id: u64,
    genre: Option<ValueGenre>,
    label: String,
    value_type: ValueType,
    value: ValueContent,
    units: String,
}

// FTR: big id is (uint64) (((uint64) m_id1 << 32) | m_id);

// AKA m_id
fn get_id0_from_id(id: u64) -> u32 {
    id as u32
}

// AKA m_id1
#[allow(unused)]
fn get_id1_from_id(id: u64) -> u32 {
    (id >> 32) as u32
}

fn get_genre(id: u32) -> Option<ValueGenre> {
    let genre: u8 = ((id & 0x00c00000) >> 22) as u8;
    genre.try_into().ok()
}

fn create_vid(home_id: u32, id: u64) -> extern_value_id::ValueID {
    unsafe { extern_value_id::value_id_from_packed_id(home_id, id) }
}

fn get_value_as_string(id: &extern_value_id::ValueID) -> Result<String> {
    // The underlying C++ lib returns a value for any type.
    let manager_ptr = unsafe { extern_manager::get() };
    let mut raw_string: *mut c_char = ptr::null_mut();

    let res = unsafe {
        extern_manager::get_value_as_string(
            manager_ptr,
            id,
            &mut raw_string,
            rust_string_creator,
        )
    };

    if res {
        Ok(recover_string(raw_string))
    } else {
        Err(Error::GetError(GetSetError::APIError("as_string")))
    }        
}

macro_rules! get_low_level_value {
    ($($typ:ty, $default:expr, $target:ident, $funci:ident, $manager_ptr:ident, $id:ident),+) => {
        $(
            {
                let mut val: $typ = $default;
                let res = unsafe {
                    extern_manager::$funci($manager_ptr, $id, &mut val)
                };
                if res {
                    Some(ValueContent::$target(val))
                } else {
                    None
                }
            }
        )*
    };
}

macro_rules! get_low_level_value_primitive {
    ($($typ:ty, $default:expr, $funci:ident, $manager_ptr:ident, $id:ident),+) => {
        $(
            {
                let mut val: $typ = $default;
                // FIXME: Simplyfy as soon as bool.then_some() is no more nightly
                if unsafe {
                    extern_manager::$funci($manager_ptr, $id, &mut val)
                } {
                    Some(val)
                } else {
                    None
                }
            }
        )*
    };
}

fn extract_value(id: &extern_value_id::ValueID, value_type: ValueType) -> Option<ValueContent> {
    let manager_ptr = unsafe { extern_manager::get() };

    match value_type {
        ValueType::Bool => {
            get_low_level_value!(bool, false, Bool, get_value_as_bool, manager_ptr, id)
        },
        ValueType::Byte => {
            get_low_level_value!(u8, 0, Byte, get_value_as_byte, manager_ptr, id)
        },
        ValueType::Decimal => {
            if let Some(value) = get_low_level_value_primitive!(f32, 0.00, get_value_as_float, manager_ptr, id) {                
                let precision: u8 = get_low_level_value_primitive!(u8, 2, get_value_float_precision, manager_ptr, id).unwrap_or(0);
                return Some(ValueContent::Decimal(DecimalValue::from_f32(value, precision)));
            }
            None
        },
        ValueType::Int => {
            get_low_level_value!(i32, 0, Int, get_value_as_int, manager_ptr, id)
        },
        ValueType::List => {
            let nearly_unsupported = ValueList { id: id.clone() };
            Some(ValueContent::String(nearly_unsupported.selection_as_string().unwrap_or("".into())))
        },
        ValueType::Short => {
            get_low_level_value!(i16, 0, Short, get_value_as_short, manager_ptr, id)
        },
        ValueType::String => {
            Some(ValueContent::String(get_value_as_string(id).unwrap_or("".into())))
        },
        ValueType::Button => {
            get_low_level_value!(bool, false, Button, get_value_as_bool, manager_ptr, id)
        },
        // FIXME: List, Schedule and Raw are still unsupported here as I have currently no idea
        //        of what exactly they should carry and I have no usage for that atm so I won't
        //        invest time in this for this moment.
        _ => None
    }
}

impl ValueID {
    pub fn from_packed_id(home_id: u32, id: u64) -> ValueID {
        let vid = create_vid(home_id, id);

        let label: String = recover_string(unsafe {
            let manager_ptr = extern_manager::get();
            extern_manager::get_value_label(manager_ptr, &vid, rust_string_creator)
        });

        let temp_type: Option<ValueType> = (get_id0_from_id(id) as u8 & 0x0F).try_into().ok();
        let value_type = match temp_type {
            Some(value) => value,
            None => ValueType::Unknown,
        };

        let units = recover_string(unsafe {
            let manager_ptr = extern_manager::get();
            extern_manager::get_value_units(manager_ptr, &vid, rust_string_creator)
        });

        let value = extract_value(&create_vid(home_id, id), value_type).unwrap_or(ValueContent::Unknown);

        ValueID {
            home_id: home_id,
            id: id,
            genre: get_genre(get_id0_from_id(id)),
            label,
            value_type,
            value,
            units,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn value(&self) -> &ValueContent {
        &self.value
    }

    pub fn units(&self) -> &str {
        &self.units
    }

    pub fn as_ozw_vid(&self) -> extern_value_id::ValueID {
        create_vid(self.home_id, self. id)
    }

    /*
    pub fn from_values(
        home_id: u32,
        node_id: u8,
        genre: ValueGenre,
        command_class_id: u8,
        instance: u8,
        value_index: u8,
        value_type: ValueType,
    ) -> ValueID {
        let ozw_vid = unsafe {
            extern_value_id::value_id_from_values(
                home_id,
                node_id,
                genre,
                command_class_id,
                instance,
                value_index,
                value_type,
            )
        };
        let id = unsafe { extern_value_id::value_id_get_id(&ozw_vid) };
        let genre = get_genre(get_id0_from_id(id));
        ValueID {
            home_id: unsafe { extern_value_id::value_id_get_home_id(&ozw_vid) },
            id,
            genre,
            label: "".into(),
        }
    }
    */

    // instance methods
    pub fn get_controller(&self) -> Controller {
        Controller::new(self.home_id)
    }

    pub fn get_node(&self) -> Node {
        Node::from_id(self.home_id, self.get_node_id())
    }

    pub fn get_home_id(&self) -> u32 {
        self.home_id
    }

    pub fn get_node_id(&self) -> u8 {
        unsafe { extern_value_id::value_id_get_node_id(&self.as_ozw_vid()) }
    }

    pub fn get_genre(&self) -> Option<ValueGenre> {
        // unsafe { extern_value_id::value_id_get_genre(&self.as_ozw_vid()) }
        self.genre
    }

    pub fn get_command_class_id(&self) -> u8 {
        unsafe { extern_value_id::value_id_get_command_class_id(&self.as_ozw_vid()) }
    }

    pub fn get_command_class(&self) -> Option<CommandClass> {
        CommandClass::from_u8(self.get_command_class_id())
    }

    pub fn get_instance(&self) -> u8 {
        unsafe { extern_value_id::value_id_get_instance(&self.as_ozw_vid()) }
    }

    pub fn get_index(&self) -> u8 {
        unsafe { extern_value_id::value_id_get_index(&self.as_ozw_vid()) }
    }

    pub fn get_type(&self) -> ValueType {
        self.value_type
        // unsafe { extern_value_id::value_id_get_type(&self.as_ozw_vid()) }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn as_raw(&self) -> Result<Box<Vec<u8>>> {
        if self.get_type() == ValueType::Raw {
            let mut raw_ptr: *mut Vec<u8> = ptr::null_mut();
            let raw_ptr_c_void = &mut raw_ptr as *mut *mut _ as *mut *mut c_void;

            let manager_ptr = unsafe { extern_manager::get() };
            let res = unsafe {
                extern_manager::get_value_as_raw(
                    manager_ptr,
                    &self.as_ozw_vid(),
                    raw_ptr_c_void,
                    rust_vec_creator::<u8>,
                )
            };

            if res {
                Ok(recover_vec(raw_ptr))
            } else {
                Err(Error::GetError(GetSetError::APIError("as_raw")))
            }
        } else {
            Err(Error::GetError(GetSetError::WrongType))
        }
    }

    // TODO: ?
    pub fn as_list(&self) -> Result<ValueList> {
        if self.get_type() == ValueType::List {
            Ok(ValueList { id: create_vid(self.home_id, self.id) })
        } else {
            Err(Error::GetError(GetSetError::WrongType))
        }
    }

    pub fn set_bool(&self, value: bool) -> Result<()> {
        match self.get_type() {
            ValueType::Bool | ValueType::Button => {
                let manager_ptr = unsafe { extern_manager::get() };
                res_to_result(unsafe {
                    extern_manager::set_value_bool(manager_ptr, &self.as_ozw_vid(), value)
                })
                .or(Err(Error::SetError(GetSetError::APIError("set_bool"))))
            }
            _ => Err(Error::SetError(GetSetError::WrongType)),
        }
    }

    pub fn set_byte(&self, value: u8) -> Result<()> {
        if self.get_type() == ValueType::Byte {
            let manager_ptr = unsafe { extern_manager::get() };
            res_to_result(unsafe {
                extern_manager::set_value_byte(manager_ptr, &self.as_ozw_vid(), value)
            })
            .or(Err(Error::SetError(GetSetError::APIError("set_byte"))))
        } else {
            Err(Error::SetError(GetSetError::WrongType))
        }
    }

    pub fn set_float(&self, value: f32) -> Result<()> {
        if self.get_type() == ValueType::Decimal {
            let manager_ptr = unsafe { extern_manager::get() };
            res_to_result(unsafe {
                extern_manager::set_value_float(manager_ptr, &self.as_ozw_vid(), value)
            })
            .or(Err(Error::SetError(GetSetError::APIError("set_float"))))
        } else {
            Err(Error::SetError(GetSetError::WrongType))
        }
    }

    pub fn set_int(&self, value: i32) -> Result<()> {
        if self.get_type() == ValueType::Int {
            let manager_ptr = unsafe { extern_manager::get() };
            res_to_result(unsafe {
                extern_manager::set_value_int(manager_ptr, &self.as_ozw_vid(), value)
            })
            .or(Err(Error::SetError(GetSetError::APIError("set_int"))))
        } else {
            Err(Error::SetError(GetSetError::WrongType))
        }
    }

    pub fn set_short(&self, value: i16) -> Result<()> {
        if self.get_type() == ValueType::Short {
            let manager_ptr = unsafe { extern_manager::get() };
            res_to_result(unsafe {
                extern_manager::set_value_short(manager_ptr, &self.as_ozw_vid(), value)
            })
            .or(Err(Error::SetError(GetSetError::APIError("set_short"))))
        } else {
            Err(Error::SetError(GetSetError::WrongType))
        }
    }

    pub fn set_string(&self, value: &str) -> Result<()> {
        // The underlying C++ lib accepts strings for all types
        let manager_ptr = unsafe { extern_manager::get() };
        let c_string = CString::new(value)?;
        res_to_result(unsafe {
            extern_manager::set_value_string(manager_ptr, &self.as_ozw_vid(), c_string.as_ptr())
        })
        .or(Err(Error::SetError(GetSetError::APIError("set_string"))))
    }

    pub fn set_raw(&self, value: &Vec<u8>) -> Result<()> {
        if self.get_type() == ValueType::Raw && value.len() < 256 {
            let manager_ptr = unsafe { extern_manager::get() };
            res_to_result(unsafe {
                extern_manager::set_value_raw(
                    manager_ptr,
                    &self.as_ozw_vid(),
                    value.as_ptr(),
                    value.len() as u8,
                )
            })
            .or(Err(Error::SetError(GetSetError::APIError("set_raw"))))
        } else {
            Err(Error::SetError(GetSetError::WrongType))
        }
    }

    pub fn set_list_selection_string(&self, value: &str) -> Result<()> {
        if self.get_type() == ValueType::List {
            let c_string = CString::new(value)?;
            let manager_ptr = unsafe { extern_manager::get() };
            res_to_result(unsafe {
                extern_manager::set_value_list_selection_string(
                    manager_ptr,
                    &self.as_ozw_vid(),
                    c_string.as_ptr(),
                )
            })
            .or(Err(Error::SetError(GetSetError::APIError(
                "set_list_selection_string",
            ))))
        } else {
            Err(Error::SetError(GetSetError::WrongType))
        }
    }

    pub fn get_label(&self) -> String {
        recover_string(unsafe {
            let manager_ptr = extern_manager::get();
            extern_manager::get_value_label(manager_ptr, &self.as_ozw_vid(), rust_string_creator)
        })
    }

    pub fn set_label(&self, str: &str) -> Result<()> {
        unsafe {
            let manager_ptr = extern_manager::get();
            let c_string = CString::new(str)?.as_ptr();
            extern_manager::set_value_label(manager_ptr, &self.as_ozw_vid(), c_string);
            Ok(())
        }
    }

    pub fn get_units(&self) -> String {
        recover_string(unsafe {
            let manager_ptr = extern_manager::get();
            extern_manager::get_value_units(manager_ptr, &self.as_ozw_vid(), rust_string_creator)
        })
    }

    pub fn set_units(&self, str: &str) -> Result<()> {
        unsafe {
            let manager_ptr = extern_manager::get();
            let c_string = CString::new(str)?.as_ptr();
            extern_manager::set_value_units(manager_ptr, &self.as_ozw_vid(), c_string);
            Ok(())
        }
    }

    pub fn get_help(&self) -> String {
        recover_string(unsafe {
            let manager_ptr = extern_manager::get();
            extern_manager::get_value_help(manager_ptr, &self.as_ozw_vid(), rust_string_creator)
        })
    }

    pub fn set_help(&self, str: &str) -> Result<()> {
        unsafe {
            let manager_ptr = extern_manager::get();
            let c_string = CString::new(str)?.as_ptr();
            extern_manager::set_value_help(manager_ptr, &self.as_ozw_vid(), c_string);
            Ok(())
        }
    }

    pub fn get_min(&self) -> i32 {
        unsafe {
            let manager_ptr = extern_manager::get();
            extern_manager::get_value_min(manager_ptr, &self.as_ozw_vid())
        }
    }

    pub fn get_max(&self) -> i32 {
        unsafe {
            let manager_ptr = extern_manager::get();
            extern_manager::get_value_max(manager_ptr, &self.as_ozw_vid())
        }
    }

    pub fn is_read_only(&self) -> bool {
        unsafe {
            let manager_ptr = extern_manager::get();
            extern_manager::is_value_read_only(manager_ptr, &self.as_ozw_vid())
        }
    }

    pub fn is_write_only(&self) -> bool {
        unsafe {
            let manager_ptr = extern_manager::get();
            extern_manager::is_value_write_only(manager_ptr, &self.as_ozw_vid())
        }
    }

    pub fn is_set(&self) -> bool {
        unsafe {
            let manager_ptr = extern_manager::get();
            extern_manager::is_value_set(manager_ptr, &self.as_ozw_vid())
        }
    }

    pub fn is_polled(&self) -> bool {
        unsafe {
            let manager_ptr = extern_manager::get();
            extern_manager::is_value_polled(manager_ptr, &self.as_ozw_vid())
        }
    }
}

impl fmt::Display for ValueID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let node = Node::from_id(self.get_home_id(), self.get_node_id());
        let mut node_name = node.get_name();
        if node_name.is_empty() {
            node_name = node.get_product_name();
        }

        let read_write = if self.is_read_only() {
            "R"
        } else if self.is_write_only() {
            "W"
        } else {
            "RW"
        };

        f.pad(&format!("HomeId: {:08x} ID: {:016x} NodeId: {:3} {:30} CC: ({:3}) {:20} Type: {:8} Label: {:20} Value: {:8} ({})",
                       self.get_home_id(),
                       self.get_id(),
                       self.get_node_id(),
                       node_name,
                       self.get_command_class_id(),
                       self.get_command_class().map_or(String::from("???"), |cc| cc.to_string()),
                       self.get_type(),
                       self.get_label(),
                       self.value().to_string(),
                       read_write,
                      )
              )
    }
}

impl fmt::Debug for ValueID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ValueID {{ home_id: {:?}, node_id: {:?}, genre: {:?}({:?}), command_class: {:?}, \
                   instance: {:?}, index: {:?}, type: {:?}, id: {:?}, \
                   label: {:?}, units: {:?}, help: {:?}, min: {:?}, max: {:?}, is_read_only: {:?}, \
                   is_write_only: {:?}, is_set: {:?}, is_polled: {:?}, \
                   value: {:?} \
                   }}",
               self.get_home_id(),
               self.get_node_id(),
               self.get_genre(),
               self.genre,
               self.get_command_class(),
               self.get_instance(),
               self.get_index(),
               self.get_type(),
               self.get_id(),
               self.get_label(),
               self.get_units(),
               self.get_help(),
               self.get_min(),
               self.get_max(),
               self.is_read_only(),
               self.is_write_only(),
               self.is_set(),
               self.is_polled(),
               self.value(),
        )
    }
}

use std::cmp::{self, Ordering};

impl cmp::PartialOrd for ValueID {
    fn partial_cmp(&self, other: &ValueID) -> Option<Ordering> {
        let is_less_than =
            unsafe { extern_value_id::value_id_less_than(&self.as_ozw_vid(), &other.as_ozw_vid()) };
        if is_less_than {
            Some(Ordering::Less)
        } else if self.eq(other) {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl cmp::Ord for ValueID {
    fn cmp(&self, other: &ValueID) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

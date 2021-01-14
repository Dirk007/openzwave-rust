use libc::c_char;
use utils::RustStringCreator;
use value_classes::value_id::ValueID;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum NotificationType {
    ValueAdded,
    ValueRemoved,
    ValueChanged,
    ValueRefreshed,
    Group,
    NodeNew,
    NodeAdded,
    NodeRemoved,
    NodeProtocolInfo,
    NodeNaming,
    NodeEvent,
    PollingDisabled,
    PollingEnabled,
    SceneEvent,
    CreateButton,
    DeleteButton,
    ButtonOn,
    ButtonOff,
    DriverReady,
    DriverFailed,
    DriverReset,
    EssentialNodeQueriesComplete,
    NodeQueriesComplete,
    AwakeNodesQueried,
    AllNodesQueriedSomeDead,
    AllNodesQueried,
    Notification,
    DriverRemoved,
    ControllerCommand,
    NodeReset,
    UserAlerts,
    ManufacturerSpecificDBReady,
}

impl From<u8> for NotificationType {
    fn from(n: u8) -> NotificationType {
        match n {
            0 => Self::ValueAdded,
            1 => Self::ValueRemoved,
            2 => Self::ValueChanged,
            3 => Self::ValueRefreshed,
            4 => Self::Group,
            5 => Self::NodeNew,
            6 => Self::NodeAdded,
            7 => Self::NodeRemoved,
            8 => Self::NodeProtocolInfo,
            9 => Self::NodeNaming,
            10 => Self::NodeEvent,
            11 => Self::PollingDisabled,
            12 => Self::PollingEnabled,
            13 => Self::SceneEvent,
            14 => Self::CreateButton,
            15 => Self::DeleteButton,
            16 => Self::ButtonOn,
            17 => Self::ButtonOff,
            18 => Self::DriverReady,
            19 => Self::DriverFailed,
            20 => Self::DriverReset,
            21 => Self::EssentialNodeQueriesComplete,
            22 => Self::NodeQueriesComplete,
            23 => Self::AwakeNodesQueried,
            24 => Self::AllNodesQueriedSomeDead,
            25 => Self::AllNodesQueried,
            26 => Self::Notification,
            27 => Self::DriverRemoved,
            28 => Self::ControllerCommand,
            29 => Self::NodeReset,
            30 => Self::UserAlerts,
            31 => Self::ManufacturerSpecificDBReady,
            _ => {
                panic!(format!("Unknown NotificationCode: {}", n));
            }
        }
    }
}

c_like_enum! {
    NotificationCode {
        MsgComplete = 0,
        Timeout = 1,
        NoOperation = 2,
        Awake = 3,
        Sleep = 4,
        Dead = 5,
        Alive = 6
    }
}

use std::fmt;
impl fmt::Display for NotificationCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

c_like_enum! {
    ControllerState {
        Normal = 0,     // No command in progress.
        Starting = 1,   // The command is starting.
        Cancel = 2,     // The command was cancelled.
        Error = 3,      // Command invocation had error(s) and was aborted.
        Waiting = 4,    // Controller is waiting for a user action.
        Sleeping = 5,   // Controller command is on a sleep queue wait for device.
        InProgress = 6, // The controller is communicating with the other device to carry out the command.
        Completed = 7,  // The command has completed successfully.
        Failed = 8,     // The command has failed.
        NodeOK = 9,     // Used only with ControllerCommand_HasNodeFailed to indicate that the controller thinks the node is OK.
        NodeFailed = 10 // Used only with ControllerCommand_HasNodeFailed to indicate that the controller thinks the node has failed.
    }
}

impl fmt::Display for ControllerState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&format!("{:?}", self))
    }
}

c_like_enum! {
    ControllerError {
        None = 0,           // No Error
        ButtonNotFound = 1, // Button
        NodeNotFound = 2,   // Button
        NotBridge = 3,      // Button
        NotSUC = 4,         // CreateNewPrimary
        NotSecondary = 5,   // CreateNewPrimary
        NotPrimary = 6,     // RemoveFailedNode, AddNodeToNetwork
        IsPrimary = 7,      // ReceiveConfiguration
        NotFound = 8,       // RemoveFailedNode
        Busy = 9,           // RemoveFailedNode, RequestNetworkUpdate
        Failed = 10,        // RemoveFailedNode, RequestNetworkUpdate
        Disabled = 11,      // RequestNetworkUpdate error
        Overflow = 12       // RequestNetworkUpdate error
    }
}

impl fmt::Display for ControllerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&format!("{:?}", self))
    }
}

pub enum Notification {}
extern "C" {
    pub fn notification_get_type(notification: *const Notification) -> u8;
    pub fn notification_get_home_id(notification: *const Notification) -> u32;
    pub fn notification_get_node_id(notification: *const Notification) -> u8;
    pub fn notification_get_value_id(notification: *const Notification) -> u64;
    pub fn notification_get_group_idx(notification: *const Notification) -> u8;
    pub fn notification_get_event(notification: *const Notification) -> u8;
    pub fn notification_get_button_id(notification: *const Notification) -> u8;
    pub fn notification_get_scene_id(notification: *const Notification) -> u8;
    pub fn notification_get_notification(notification: *const Notification) -> u8;
    pub fn notification_get_byte(notification: *const Notification) -> u8;
    pub fn notification_get_as_string(
        notification: *const Notification,
        rust_string_creator: RustStringCreator,
    ) -> *const c_char;
}

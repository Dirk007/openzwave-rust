use libc::c_char;
use utils::RustStringCreator;
use value_classes::value_id::ValueID;

c_like_enum! {
    NotificationType {
        ValueAdded = 0,
        ValueRemoved = 1,
        ValueChanged = 2,
        ValueRefreshed = 3,
        Group = 4,
        NodeNew = 5,
        NodeAdded = 6,
        NodeRemoved = 7,
        NodeProtocolInfo = 8,
        NodeNaming = 9,
        NodeEvent = 10,
        PollingDisabled = 11,
        PollingEnabled = 12,
        SceneEvent = 13,
        CreateButton = 14,
        DeleteButton = 15,
        ButtonOn = 16,
        ButtonOff = 17,
        DriverReady = 18,
        DriverFailed = 19,
        DriverReset = 20,
        EssentialNodeQueriesComplete = 21,
        NodeQueriesComplete = 22,
        AwakeNodesQueried = 23,
        AllNodesQueriedSomeDead = 24,
        AllNodesQueried = 25,
        Notification = 26,
        DriverRemoved = 27,
        ControllerCommand = 28,
        NodeReset = 29,
        UserAlerts = 30,
        ManufacturerSpecificDBReady = 31,

        Unknown = 255
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

pub use ffi::notification::{ ControllerState, ControllerError, NotificationType, NotificationCode, Notification as ExternNotification };
use ffi::notification as extern_notification;
use ffi::value_classes::value_id as extern_value_id;
use value_classes::value_id::ValueID;
use libc::c_char;
use ffi::utils::{ rust_string_creator, recover_string };
use node::Node;
use controller::Controller;

#[derive(Debug)]
pub enum NotificationValue {
    Group(u8),
    Button(u8),
    Scene(u8),
    State(ControllerState),
    Report(NotificationCode),
}

#[derive(Debug)]
pub struct Notification {
    ptr: *const ExternNotification,
    pub notification_type: NotificationType,
    pub home_id: u32,
    pub node_id: u8,
    pub value_id: Option<ValueID>,
    pub value: Option<NotificationValue>,
    pub event: Option<u8>,
}

impl Into<String> for Notification {
    fn into(self) -> String {
        recover_string(
            unsafe {
                extern_notification::notification_get_as_string(self.ptr, rust_string_creator)
            } as *mut c_char
        )
    }
}

impl Into<u8> for Notification {
    fn into(self) -> u8 {
        unsafe { extern_notification::notification_get_byte(self.ptr) }
    }
}

impl Notification {
    pub fn new(ptr: *const ExternNotification) -> Notification {
        log::info!("Creating from {:?}", ptr);

        let home_id = unsafe { extern_notification::notification_get_home_id(ptr) };
        let node_id = unsafe { extern_notification::notification_get_node_id(ptr) };
        let notification_type = unsafe { extern_notification::notification_get_type(ptr).into() };

        log::info!("HomeId {:x}, NodeId {:x}", home_id, node_id);

        let foo = Notification {
            ptr,
            notification_type,
            home_id,
            node_id,
            value_id: match home_id {
                0 => None,
                _ => unsafe {
                        let ozw_vid = extern_notification::notification_get_value_id(ptr);
                            if ozw_vid >> 32 > 0 {
                                log::info!("OZW VID {:x}", ozw_vid);
                                Some(ValueID::from_packed_id(home_id, ozw_vid))
                            } else {
                                None
                            }
                    }
            },
            value: match notification_type {
                NotificationType::Group => Some(NotificationValue::Group(unsafe { extern_notification::notification_get_group_idx(ptr) })),
                NotificationType::CreateButton | NotificationType::DeleteButton | NotificationType::ButtonOn | NotificationType::ButtonOff => Some(NotificationValue::Button(unsafe { extern_notification::notification_get_button_id(ptr) })),
                NotificationType::SceneEvent => Some(NotificationValue::Scene(unsafe { extern_notification::notification_get_scene_id(ptr) })),
                // TODO: This is wrong? m_command instead of m_byte used in openzwave-sys/open-zwave/cpp/src/Notification.h
                NotificationType::ControllerCommand => {
                    if let Some(state) = ControllerState::from_u8(unsafe { extern_notification::notification_get_notification(ptr) }) {
                        Some(NotificationValue::State(state))
                    } else {
                        None
                    }
                },
                NotificationType::Notification => {
                    if let Some(code) = NotificationCode::from_u8(unsafe { extern_notification::notification_get_notification(ptr) }) {
                        Some(NotificationValue::Report(code))
                    } else {
                        None
                    }
                },
                _ => None,
            },
            event: match notification_type {
                NotificationType::NodeEvent | NotificationType::ControllerCommand => Some(unsafe { extern_notification::notification_get_event(ptr) }),
                _ => None
            },
        };
        log::info!("Created from {:?}", ptr);
        log::info!("Created from {:?} -> {:?}", ptr, foo);
        foo
    }

    pub fn get_controller(&self) -> Controller {
        Controller::new(self.home_id)
    }
}

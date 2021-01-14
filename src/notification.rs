pub use ffi::notification::{ ControllerState, ControllerError, NotificationType, NotificationCode, Notification as ExternNotification };
use ffi::notification as extern_notification;
use ffi::value_classes::value_id as extern_value_id;
use value_classes::value_id::ValueID;
use libc::c_char;
use ffi::utils::{ rust_string_creator, recover_string };
use node::Node;
use controller::Controller;

#[derive(Debug)]
pub struct Notification {
    ptr: *const ExternNotification,
    pub notification_type: NotificationType,
    pub home_id: u32,
    pub node_id: u8,
    pub value_id: Option<ValueID>,
    pub group_idx: Option<u8>,
    pub event: Option<u8>,
    pub button_id: Option<u8>,
    pub scene_id: Option<u8>,
    pub controller_state: Option<ControllerState>,
    pub notification_code: Option<NotificationCode>,
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
            group_idx: match notification_type {
                NotificationType::Group =>
                    Some(unsafe { extern_notification::notification_get_group_idx(ptr) }),
                _ => None
            },
            event: match notification_type {
                NotificationType::NodeEvent | NotificationType::ControllerCommand => Some(unsafe { extern_notification::notification_get_event(ptr) }),
                _ => None
            },
            button_id: match notification_type {
                NotificationType::CreateButton | NotificationType::DeleteButton | NotificationType::ButtonOn | NotificationType::ButtonOff => Some(unsafe { extern_notification::notification_get_button_id(ptr) }),
                _ => None
            },
            scene_id: match notification_type {
                NotificationType::SceneEvent => Some(unsafe { extern_notification::notification_get_scene_id(ptr) }),
                _ => None
            },
            controller_state: match notification_type {
                NotificationType::ControllerCommand => ControllerState::from_u8(unsafe { extern_notification::notification_get_notification(ptr) }),
                _ => None
            },
            notification_code: match notification_type {
                NotificationType::Notification => NotificationCode::from_u8(unsafe { extern_notification::notification_get_notification(ptr) }),
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

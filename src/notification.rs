pub use ffi::notification::{ ControllerState, ControllerError, NotificationType, NotificationCode, Notification as ExternNotification };
use ffi::notification as extern_notification;
use crate::value_classes::value_id::ValueID;
use crate::controller::Controller;

#[derive(Debug, Clone)]
pub enum NotificationValue {
    Group(u8),
    Button(u8),
    Scene(u8),
    State(ControllerState),
    Report(NotificationCode),
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub notification_type: NotificationType,
    pub home_id: u32,
    pub node_id: u8,
    pub value_id: Option<ValueID>,
    pub value: Option<NotificationValue>,
    pub event: Option<u8>,
}

#[inline(always)]
fn is_valid_value_id(value: u64) -> bool {
    value >> 32 > 0
}

impl Notification {
    pub fn new(ptr: *const ExternNotification) -> Self {

        let home_id = unsafe { extern_notification::notification_get_home_id(ptr) };
        let node_id = unsafe { extern_notification::notification_get_node_id(ptr) };
        let notification_type = match NotificationType::from_u8(unsafe { extern_notification::notification_get_type(ptr) }) {
            Some(converted) => converted,
            None => NotificationType::Unknown,
        };

        Self {
            notification_type,
            home_id,
            node_id,
            value_id: match home_id {
                0 => None,
                _ => unsafe {
                        let ozw_vid = extern_notification::notification_get_value_id(ptr);
                            if is_valid_value_id(ozw_vid) {
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
        }
    }

    pub fn get_controller(&self) -> Controller {
        Controller::new(self.home_id)
    }
}

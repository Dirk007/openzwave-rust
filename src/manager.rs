use crate::error::{Error, Result};
use crate::notification::{ExternNotification, Notification};
use crate::options::Options;
use crate::value_classes::value_id::ValueID;
use ffi::manager as extern_manager;
use ffi::utils as extern_utils;
use ffi::utils::res_to_result;
use libc::c_void;
use std::ffi::CString;

pub struct Manager {
    pub ptr: *mut extern_manager::Manager,
    #[allow(dead_code)]
    options: Options, //< this is a false positive
    watchers: Vec<Option<Box<WatcherWrapper>>>,
}

unsafe impl Send for Manager {}
unsafe impl Sync for Manager {}

pub trait NotificationWatcher: Sync {
    fn on_notification(&self, notification: &Notification);
}

struct WatcherWrapper {
    watcher: Box<dyn NotificationWatcher>,
}

// watcher is actually a Box<WatcherWrapper>
extern "C" fn watcher_cb(notification: *const ExternNotification, watcher: *const c_void) {
    let watcher_wrapper: &WatcherWrapper = unsafe { &*(watcher as *const WatcherWrapper) };
    let rust_notification = Notification::new(notification);
    // log::info!("watcher_cb -> {:?}", rust_notification.notification_type);
    watcher_wrapper.watcher.on_notification(&rust_notification);
}

impl Manager {
    pub fn create(mut options: Options) -> Result<Manager> {
        options.lock()?;
        let external_manager = unsafe { extern_manager::manager_create() };
        if external_manager.is_null() {
            Err(Error::OptionsAreNotLocked("Manager::create"))
        } else {
            Ok(Manager {
                ptr: external_manager,
                options: options,
                watchers: Vec::with_capacity(1),
            })
        }
    }

    pub fn get_node_manufacturer_id(&self, home_id: u32, node_id: u8) -> String {
        let raw = unsafe {
            extern_manager::get_node_manufacturer_id(
                self.ptr,
                home_id,
                node_id,
                extern_utils::rust_string_creator,
            )
        };
        extern_utils::recover_string(raw)
    }

    pub fn get_node_manufacturer_name(&self, home_id: u32, node_id: u8) -> String {
        let raw = unsafe {
            extern_manager::get_node_manufacturer_name(
                self.ptr,
                home_id,
                node_id,
                extern_utils::rust_string_creator,
            )
        };
        extern_utils::recover_string(raw)
    }

    pub fn get_node_product_id(&self, home_id: u32, node_id: u8) -> String {
        let raw = unsafe {
            extern_manager::get_node_product_id(
                self.ptr,
                home_id,
                node_id,
                extern_utils::rust_string_creator,
            )
        };
        extern_utils::recover_string(raw)
    }

    pub fn get_node_product_name(&self, home_id: u32, node_id: u8) -> String {
        let raw = unsafe {
            extern_manager::get_node_product_name(
                self.ptr,
                home_id,
                node_id,
                extern_utils::rust_string_creator,
            )
        };
        extern_utils::recover_string(raw)
    }

    pub fn set_value_byte(&self, vid: &ValueID, value: u8) -> bool {
        unsafe { extern_manager::manager_set_value_byte(self.ptr, vid.vid(), value) }
    }

    pub fn request_node_state(&self, home_id: u32, node_id: u8) -> bool {
        unsafe { extern_manager::manager_request_node_state(self.ptr, home_id, node_id) }
    }

    pub fn request_all_config_params(&self, home_id: u32, node_id: u8) {
        unsafe { extern_manager::manager_request_all_config_params(self.ptr, home_id, node_id) }
    }

    pub fn reset_controller(&self, home_id: u32) {
        unsafe {
            extern_manager::reset_controller(self.ptr, home_id);
        }
    }

    pub fn soft_reset_controller(&self, home_id: u32) {
        unsafe {
            extern_manager::soft_reset_controller(self.ptr, home_id);
        }
    }

    pub fn cancel_controller_command(&self, home_id: u32) {
        unsafe {
            extern_manager::cancel_controller_command(self.ptr, home_id);
        }
    }

    pub fn add_node(&self, home_id: u32, secure: bool) -> Result<()> {
        res_to_result(unsafe { extern_manager::manager_add_node(self.ptr, home_id, secure) })
            .or(Err(Error::InvalidParameter("home_id", "Manager::add_node")))
    }

    pub fn remove_node(&self, home_id: u32) -> Result<()> {
        res_to_result(unsafe { extern_manager::manager_remove_node(self.ptr, home_id) }).or(Err(
            Error::InvalidParameter("home_id", "Manager::remove_node"),
        ))
    }

    pub fn test_network(&self, home_id: u32, count: u32) {
        unsafe {
            extern_manager::test_network(self.ptr, home_id, count);
        }
    }

    pub fn test_network_node(&self, home_id: u32, node_id: u8, count: u32) {
        unsafe {
            extern_manager::test_network_node(self.ptr, home_id, node_id, count);
        }
    }

    pub fn heal_network(&self, home_id: u32, do_rr: bool) {
        unsafe {
            extern_manager::heal_network(self.ptr, home_id, do_rr);
        }
    }

    pub fn heal_network_node(&self, home_id: u32, node_id: u8, do_rr: bool) {
        unsafe {
            extern_manager::heal_network_node(self.ptr, home_id, node_id, do_rr);
        }
    }

    pub fn add_watcher<T: 'static + NotificationWatcher>(&mut self, watcher: T) -> Result<usize> {
        let watcher_wrapper = Box::new(WatcherWrapper {
            watcher: Box::new(watcher),
        });

        let watcher_ptr: *const c_void = &*watcher_wrapper as *const _ as *const c_void;
        let api_res =
            unsafe { extern_manager::manager_add_watcher(self.ptr, watcher_cb, watcher_ptr) };

        if api_res {
            let position = self.watchers.len();
            self.watchers.push(Some(watcher_wrapper));
            Ok(position)
        } else {
            Err(Error::APIError(
                "Could not add a watcher: it's already added",
            ))
        }
    }

    pub fn remove_watcher(&mut self, position: usize) -> Result<()> {
        let wrapper = self.watchers[position].take();

        if let Some(mut wrapper) = wrapper {
            let result = self.remove_watcher_impl(&mut wrapper);
            if result.is_err() {
                // put the watcher back to the vec
                self.watchers[position] = Some(wrapper);
            }
            result
        } else {
            Err(Error::APIError("Could not find the watcher to remove"))
        }
    }

    fn remove_watcher_impl(&self, wrapper: &mut WatcherWrapper) -> Result<()> {
        let watcher_ptr: *mut c_void = wrapper as *mut _ as *mut c_void;
        res_to_result(unsafe {
            extern_manager::manager_remove_watcher(self.ptr, watcher_cb, watcher_ptr)
        })
        .or(Err(Error::APIError(
            "Could not remove a watcher as it was not added or already removed",
        )))
    }

    pub fn add_driver(&mut self, device: &str) -> Result<()> {
        let device = CString::new(device).unwrap();
        res_to_result(unsafe {
            extern_manager::manager_add_driver(
                self.ptr,
                device.as_ptr(),
                &extern_manager::ControllerInterface::Serial,
            )
        })
        .or(Err(Error::APIError(
            "Could not add the driver as it is already added",
        )))
    }

    pub fn add_usb_driver(&mut self) -> Result<()> {
        let device = CString::new("HID Controller").unwrap();
        res_to_result(unsafe {
            extern_manager::manager_add_driver(
                self.ptr,
                device.as_ptr(),
                &extern_manager::ControllerInterface::Hid,
            )
        })
        .or(Err(Error::APIError(
            "Could not add the driver as it is already added",
        )))
    }

    pub fn remove_driver(&mut self, device: &str) -> Result<()> {
        let device = CString::new(device).unwrap();
        res_to_result(unsafe { extern_manager::manager_remove_driver(self.ptr, device.as_ptr()) })
            .or(Err(Error::APIError(
                "Could not remove the driver as it was not added or already removed",
            )))
    }

    pub fn remove_usb_driver(&mut self) -> Result<()> {
        let device = CString::new("HID Controller").unwrap();
        res_to_result(unsafe { extern_manager::manager_remove_driver(self.ptr, device.as_ptr()) })
            .or(Err(Error::APIError(
                "Could not remove the driver as it was not added or already removed",
            )))
    }

    pub fn get_poll_interval(&self) -> i32 {
        unsafe { extern_manager::manager_get_poll_interval(self.ptr) }
    }

    pub fn set_poll_interval(&self, interval_ms: i32, is_between_each_poll: bool) {
        unsafe {
            extern_manager::manager_set_poll_interval(self.ptr, interval_ms, is_between_each_poll)
        }
    }

    pub fn enable_poll_with_intensity(&self, vid: &ValueID, intensity: u8) -> bool {
        unsafe {
            extern_manager::manager_enable_poll_with_intensity(self.ptr, vid.vid(), intensity)
        }
    }

    pub fn enable_poll(&self, vid: &ValueID) -> bool {
        unsafe { extern_manager::manager_enable_poll(self.ptr, vid.vid()) }
    }

    pub fn disable_poll(&self, vid: &ValueID) -> bool {
        unsafe { extern_manager::manager_disable_poll(self.ptr, vid.vid()) }
    }

    pub fn is_polled(&self, vid: &ValueID) -> bool {
        unsafe { extern_manager::manager_is_polled(self.ptr, vid.vid()) }
    }

    pub fn set_poll_intensity(&self, vid: &ValueID, intensity: u8) {
        unsafe { extern_manager::manager_set_poll_intensity(self.ptr, vid.vid(), intensity) }
    }

    pub fn get_poll_intensity(&self, vid: &ValueID) -> u8 {
        unsafe { extern_manager::manager_get_poll_intensity(self.ptr, vid.vid()) }
    }
}

impl Drop for Manager {
    fn drop(&mut self) {
        let watchers: Vec<_> = self.watchers.drain(..).collect();
        for watcher in watchers {
            if let Some(mut watcher) = watcher {
                self.remove_watcher_impl(&mut watcher).unwrap();
            }
        }

        unsafe { extern_manager::manager_destroy() }
    }
}

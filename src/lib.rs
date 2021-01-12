#![allow(non_camel_case_types)]

extern crate libc;

use std::convert::TryInto;

use libc::{c_char, c_int, c_short, c_uchar, c_uint, c_void, ssize_t, timeval};

#[repr(C)]
pub struct libusb_context {
    __private: c_void
}

#[repr(C)]
pub struct libusb_device {
    __private: c_void
}

#[repr(C)]
pub struct libusb_device_handle {
    __private: c_void
}

#[repr(C)]
pub struct libusb_version {
    pub major: u16,
    pub minor: u16,
    pub micro: u16,
    pub nano: u16,
    pub rc: *const c_char,
    pub describe: *const c_char,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct libusb_device_descriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bcdUSB: u16,
    pub bDeviceClass: u8,
    pub bDeviceSubClass: u8,
    pub bDeviceProtocol: u8,
    pub bMaxPacketSize0: u8,
    pub idVendor: u16,
    pub idProduct: u16,
    pub bcdDevice: u16,
    pub iManufacturer: u8,
    pub iProduct: u8,
    pub iSerialNumber: u8,
    pub bNumConfigurations: u8,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct libusb_config_descriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub wTotalLength: u16,
    pub bNumInterfaces: u8,
    pub bConfigurationValue: u8,
    pub iConfiguration: u8,
    pub bmAttributes: u8,
    pub bMaxPower: u8,
    pub interface: *const libusb_interface,
    pub extra: *const c_uchar,
    pub extra_length: c_int,
}

#[repr(C)]
pub struct libusb_interface {
    pub altsetting: *const libusb_interface_descriptor,
    pub num_altsetting: c_int,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct libusb_interface_descriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bInterfaceNumber: u8,
    pub bAlternateSetting: u8,
    pub bNumEndpoints: u8,
    pub bInterfaceClass: u8,
    pub bInterfaceSubClass: u8,
    pub bInterfaceProtocol: u8,
    pub iInterface: u8,
    pub endpoint: *const libusb_endpoint_descriptor,
    pub extra: *const c_uchar,
    pub extra_length: c_int,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct libusb_endpoint_descriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bEndpointAddress: u8,
    pub bmAttributes: u8,
    pub wMaxPacketSize: u16,
    pub bInterval: u8,
    pub bRefresh: u8,
    pub bSynchAddress: u8,
    pub extra: *const c_uchar,
    pub extra_length: c_int,
}

#[repr(C)]
pub struct libusb_iso_packet_descriptor {
    pub length: c_uint,
    pub actual_length: c_uint,
    pub status: c_int,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct libusb_ss_endpoint_companion_descriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bMaxBurst: u8,
    pub bmAttributes: u8,
    pub wBytesPerInterval: u16,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct libusb_bos_dev_capability_descriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub dev_capability_data: [u8;0],
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct libusb_bos_descriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub wTotalLength: u16,
    pub bNumDeviceCaps: u8,
    pub dev_capability: *mut [libusb_bos_dev_capability_descriptor;0],
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct libusb_usb_2_0_extension_descriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub bmAttributes: u32,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct libusb_ss_usb_device_capability_descriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub bmAttributes: u8,
    pub wSpeedSupported: u16,
    pub bFunctionalitySupport: u8,
    pub bU1DevExitLat: u8,
    pub bU2DevExitLat: u8,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct libusb_container_id_descriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub bReserved: u8,
    pub ContainerId: [u8; 16],
}

#[allow(non_snake_case)]
#[repr(C)]
struct libusb_control_setup {
    /// Request type. Bits 0:4 determine recipient, see
    /// libusb_request_recipient. Bits 5:6 determine type, see
    /// libusb_request_type. Bit 7 determines data transfer direction, see
    /// libusb_endpoint_direction.
    pub bmRequestType: u8,
    /// Request. If the type bits of bmRequestType are equal to
    /// libusb_request_type::LIBUSB_REQUEST_TYPE_STANDARD
    /// "LIBUSB_REQUEST_TYPE_STANDARD" then this field refers to
    /// libusb_standard_request. For other cases, use of this field is
    /// application-specific.
    pub bRequest: u8,
    /// Value. Varies according to request
    pub wValue: u16,
    /// Index. Varies according to request, typically used to pass an index or offset
    pub wIndex: u16,
    /// Number of bytes to transfer
    pub wLength: u16,
}

pub const LIBUSB_CONTROL_SETUP_SIZE: usize = std::mem::size_of::<libusb_control_setup>();

#[repr(C)]
pub struct libusb_transfer {
    pub dev_handle: *mut libusb_device_handle,
    pub flags: u8,
    pub endpoint: c_uchar,
    pub transfer_type: c_uchar,
    pub timeout: c_uint,
    pub status: c_int,
    pub length: c_int,
    pub actual_length: c_int,
    pub callback: libusb_transfer_cb_fn,
    pub user_data: *mut c_void,
    pub buffer: *mut c_uchar,
    pub num_iso_packets: c_int,
    pub iso_packet_desc: [libusb_iso_packet_descriptor; 0],
}

#[repr(C)]
pub struct libusb_pollfd {
    pub fd: c_int,
    pub events: c_short,
}


pub type libusb_hotplug_callback_handle = c_int;
pub type libusb_hotplug_callback_fn = extern "C" fn(*mut libusb_context, *mut libusb_device, libusb_hotplug_event, *mut c_void) -> c_int;
pub type libusb_transfer_cb_fn = extern "C" fn(*mut libusb_transfer);
pub type libusb_pollfd_added_cb = extern "C" fn(c_int, c_short, *mut c_void);
pub type libusb_pollfd_removed_cb = extern "C" fn(c_int, *mut c_void);
pub type libusb_log_cb = extern "C" fn(*mut libusb_context, c_int/*libusb_log_level*/, *const c_char /*str to log?*/);


// libusb_error
pub const LIBUSB_SUCCESS:             c_int = 0;
pub const LIBUSB_ERROR_IO:            c_int = -1;
pub const LIBUSB_ERROR_INVALID_PARAM: c_int = -2;
pub const LIBUSB_ERROR_ACCESS:        c_int = -3;
pub const LIBUSB_ERROR_NO_DEVICE:     c_int = -4;
pub const LIBUSB_ERROR_NOT_FOUND:     c_int = -5;
pub const LIBUSB_ERROR_BUSY:          c_int = -6;
pub const LIBUSB_ERROR_TIMEOUT:       c_int = -7;
pub const LIBUSB_ERROR_OVERFLOW:      c_int = -8;
pub const LIBUSB_ERROR_PIPE:          c_int = -9;
pub const LIBUSB_ERROR_INTERRUPTED:   c_int = -10;
pub const LIBUSB_ERROR_NO_MEM:        c_int = -11;
pub const LIBUSB_ERROR_NOT_SUPPORTED: c_int = -12;
pub const LIBUSB_ERROR_OTHER:         c_int = -99;

// libusb_transfer_status
pub const LIBUSB_TRANSFER_COMPLETED:  c_int = 0;
pub const LIBUSB_TRANSFER_ERROR:      c_int = 1;
pub const LIBUSB_TRANSFER_TIMED_OUT:  c_int = 2;
pub const LIBUSB_TRANSFER_CANCELLED:  c_int = 3;
pub const LIBUSB_TRANSFER_STALL:      c_int = 4;
pub const LIBUSB_TRANSFER_NO_DEVICE:  c_int = 5;
pub const LIBUSB_TRANSFER_OVERFLOW:   c_int = 6;

pub const LIBUSB_TRANSFER_SHORT_NOT_OK:    u8 = 1<<0;
pub const LIBUSB_TRANSFER_FREE_BUFFER :    u8 = 1<<1;
pub const LIBUSB_TRANSFER_FREE_TRANSFER :  u8 = 1<<2;
pub const LIBUSB_TRANSFER_ADD_ZERO_PACKET: u8 = 1<<3;

// libusb_capability
pub const LIBUSB_CAP_HAS_CAPABILITY:                u32 = 0x0000;
pub const LIBUSB_CAP_HAS_HOTPLUG:                   u32 = 0x0001;
pub const LIBUSB_CAP_HAS_HID_ACCESS:                u32 = 0x0100;
pub const LIBUSB_CAP_SUPPORTS_DETACH_KERNEL_DRIVER: u32 = 0x0101;


//// libusb_log_level
pub const LIBUSB_LOG_LEVEL_NONE:    c_int = 0;
pub const LIBUSB_LOG_LEVEL_ERROR:   c_int = 1;
pub const LIBUSB_LOG_LEVEL_WARNING: c_int = 2;
pub const LIBUSB_LOG_LEVEL_INFO:    c_int = 3;
pub const LIBUSB_LOG_LEVEL_DEBUG:   c_int = 4;

/// libusb_log_cb_mode
pub const LIBUSB_LOG_CB_GLOBAL:  u8 = 1 << 0;
pub const LIBUSB_LOG_CB_CONTEXT: u8 = 1 << 1;

// libusb_class_code
pub const LIBUSB_CLASS_PER_INTERFACE:       u8 = 0;
pub const LIBUSB_CLASS_AUDIO:               u8 = 1;
pub const LIBUSB_CLASS_COMM:                u8 = 2;
pub const LIBUSB_CLASS_HID:                 u8 = 3;
pub const LIBUSB_CLASS_PHYSICAL:            u8 = 5;
pub const LIBUSB_CLASS_PRINTER:             u8 = 7;
pub const LIBUSB_CLASS_IMAGE:               u8 = 6;
pub const LIBUSB_CLASS_MASS_STORAGE:        u8 = 8;
pub const LIBUSB_CLASS_HUB:                 u8 = 9;
pub const LIBUSB_CLASS_DATA:                u8 = 10;
pub const LIBUSB_CLASS_SMART_CARD:          u8 = 0x0B;
pub const LIBUSB_CLASS_CONTENT_SECURITY:    u8 = 0x0D;
pub const LIBUSB_CLASS_VIDEO:               u8 = 0x0E;
pub const LIBUSB_CLASS_PERSONAL_HEALTHCARE: u8 = 0x0F;
pub const LIBUSB_CLASS_DIAGNOSTIC_DEVICE:   u8 = 0xDC;
pub const LIBUSB_CLASS_WIRELESS:            u8 = 0xE0;
pub const LIBUSB_CLASS_APPLICATION:         u8 = 0xFE;
pub const LIBUSB_CLASS_VENDOR_SPEC:         u8 = 0xFF;


// libusb_speed
pub const LIBUSB_SPEED_UNKNOWN: c_int = 0;
pub const LIBUSB_SPEED_LOW:     c_int = 1;
pub const LIBUSB_SPEED_FULL:    c_int = 2;
pub const LIBUSB_SPEED_HIGH:    c_int = 3;
pub const LIBUSB_SPEED_SUPER:   c_int = 4;
/// The device is operating at super speed plus (10000MBit/s).
pub const LIBUSB_SPEED_SUPER_PLUS: c_int = 5;

// libusb_supported_speed
/// Low speed operation supported (1.5MBit/s).
pub const LIBUSB_LOW_SPEED_OPERATION: c_int   = 1;
/// Full speed operation supported (12MBit/s).
pub const LIBUSB_FULL_SPEED_OPERATION: c_int  = 2;
/// High speed operation supported (480MBit/s).
pub const LIBUSB_HIGH_SPEED_OPERATION: c_int  = 4;
/// Superspeed operation supported (5000MBit/s).
pub const LIBUSB_SUPER_SPEED_OPERATION: c_int = 8;


// libusb_descriptor_type
pub const LIBUSB_DT_DEVICE:                u8 = 0x01;
pub const LIBUSB_DT_CONFIG:                u8 = 0x02;
pub const LIBUSB_DT_STRING:                u8 = 0x03;
pub const LIBUSB_DT_INTERFACE:             u8 = 0x04;
pub const LIBUSB_DT_ENDPOINT:              u8 = 0x05;
pub const LIBUSB_DT_BOS:                   u8 = 0x0F;
pub const LIBUSB_DT_DEVICE_CAPABILITY:     u8 = 0x10;
pub const LIBUSB_DT_HID:                   u8 = 0x21;
pub const LIBUSB_DT_REPORT:                u8 = 0x22;
pub const LIBUSB_DT_PHYSICAL:              u8 = 0x23;
pub const LIBUSB_DT_HUB:                   u8 = 0x29;
pub const LIBUSB_DT_SUPERSPEED_HUB:        u8 = 0x2A;
pub const LIBUSB_DT_SS_ENDPOINT_COMPANION: u8 = 0x30;


// libusb_endpoint_direction
pub const LIBUSB_ENDPOINT_ADDRESS_MASK: u8 = 0x0F;
pub const LIBUSB_ENDPOINT_DIR_MASK:     u8 = 0x80;
pub const LIBUSB_ENDPOINT_IN:           u8 = 0x80;
pub const LIBUSB_ENDPOINT_OUT:          u8 = 0x00;


// libusb_transfer_type
pub const LIBUSB_TRANSFER_TYPE_MASK:        u8 = 0x03;
pub const LIBUSB_TRANSFER_TYPE_CONTROL:     u8 = 0;
pub const LIBUSB_TRANSFER_TYPE_ISOCHRONOUS: u8 = 1;
pub const LIBUSB_TRANSFER_TYPE_BULK:        u8 = 2;
pub const LIBUSB_TRANSFER_TYPE_INTERRUPT:   u8 = 3;
pub const LIBUSB_TRANSFER_TYPE_BULK_STREAM: u8 = 4;


// libusb_iso_sync_type
pub const LIBUSB_ISO_SYNC_TYPE_MASK:     u8 = 0x0C;
pub const LIBUSB_ISO_SYNC_TYPE_NONE:     u8 = 0;
pub const LIBUSB_ISO_SYNC_TYPE_ASYNC:    u8 = 1;
pub const LIBUSB_ISO_SYNC_TYPE_ADAPTIVE: u8 = 2;
pub const LIBUSB_ISO_SYNC_TYPE_SYNC:     u8 = 3;


// libusb_iso_usage_type
pub const LIBUSB_ISO_USAGE_TYPE_MASK:     u8 = 0x30;
pub const LIBUSB_ISO_USAGE_TYPE_DATA:     u8 = 0;
pub const LIBUSB_ISO_USAGE_TYPE_FEEDBACK: u8 = 1;
pub const LIBUSB_ISO_USAGE_TYPE_IMPLICIT: u8 = 2;


// libusb_request_type
pub const LIBUSB_REQUEST_TYPE_STANDARD: u8 = 0x00 << 5;
pub const LIBUSB_REQUEST_TYPE_CLASS:    u8 = 0x01 << 5;
pub const LIBUSB_REQUEST_TYPE_VENDOR:   u8 = 0x02 << 5;
pub const LIBUSB_REQUEST_TYPE_RESERVED: u8 = 0x03 << 5;


// libusb_request_recipient
pub const LIBUSB_RECIPIENT_DEVICE:    u8 = 0x00;
pub const LIBUSB_RECIPIENT_INTERFACE: u8 = 0x01;
pub const LIBUSB_RECIPIENT_ENDPOINT:  u8 = 0x02;
pub const LIBUSB_RECIPIENT_OTHER:     u8 = 0x03;


// libusb_standard_request
pub const LIBUSB_REQUEST_GET_STATUS:        u8 = 0x00;
pub const LIBUSB_REQUEST_CLEAR_FEATURE:     u8 = 0x01;
pub const LIBUSB_REQUEST_SET_FEATURE:       u8 = 0x03;
pub const LIBUSB_REQUEST_SET_ADDRESS:       u8 = 0x05;
pub const LIBUSB_REQUEST_GET_DESCRIPTOR:    u8 = 0x06;
pub const LIBUSB_REQUEST_SET_DESCRIPTOR:    u8 = 0x07;
pub const LIBUSB_REQUEST_GET_CONFIGURATION: u8 = 0x08;
pub const LIBUSB_REQUEST_SET_CONFIGURATION: u8 = 0x09;
pub const LIBUSB_REQUEST_GET_INTERFACE:     u8 = 0x0A;
pub const LIBUSB_REQUEST_SET_INTERFACE:     u8 = 0x0B;
pub const LIBUSB_REQUEST_SYNCH_FRAME:       u8 = 0x0C;
pub const LIBUSB_REQUEST_SET_SEL:           u8 = 0x30;
pub const LIBUSB_SET_ISOCH_DELAY:           u8 = 0x31;


// libusb_hotplug_event
pub type libusb_hotplug_event = c_int;
pub const LIBUSB_HOTPLUG_EVENT_DEVICE_ARRIVED: libusb_hotplug_event = 1 << 0;
pub const LIBUSB_HOTPLUG_EVENT_DEVICE_LEFT:    libusb_hotplug_event = 1 << 1;


// libusb_hotplug_flag
pub const LIBUSB_HOTPLUG_NO_FLAGS:  c_int = 0;
pub const LIBUSB_HOTPLUG_MATCH_ANY: c_int = -1;
pub const LIBUSB_HOTPLUG_ENUMERATE: c_int = 1 << 0;


extern "C" {
    pub fn libusb_get_version() -> *const libusb_version;
    pub fn libusb_has_capability(capability: u32) -> c_int;
    pub fn libusb_error_name(errcode: c_int) -> *const c_char;
    pub fn libusb_setlocale(locale: *const c_char) -> c_int;
    pub fn libusb_strerror(errcode: c_int) -> *const c_char;

    pub fn libusb_init(context: *mut *mut libusb_context) -> c_int;
    pub fn libusb_exit(context: *mut libusb_context);
    pub fn libusb_set_debug(context: *mut libusb_context, level: c_int);
    pub fn libusb_set_log_cb(context: *mut libusb_context, cd: libusb_log_cb, mode: c_int /*libusb_log_level*/);

    pub fn libusb_get_device_list(context: *mut libusb_context, list: *mut *const *mut libusb_device) -> ssize_t;
    pub fn libusb_free_device_list(list: *const *mut libusb_device, unref_devices: c_int);
    pub fn libusb_get_parent(dev: *mut libusb_device) -> *mut libusb_device;
    pub fn libusb_get_device(dev_handle: *mut libusb_device_handle) -> *mut libusb_device;

    pub fn libusb_ref_device(dev: *mut libusb_device) -> *mut libusb_device;
    pub fn libusb_unref_device(dev: *mut libusb_device);

    pub fn libusb_get_device_descriptor(dev: *const libusb_device, desc: *mut libusb_device_descriptor) -> c_int;
    pub fn libusb_get_config_descriptor(dev: *const libusb_device, index: u8, config: *mut *const libusb_config_descriptor) -> c_int;
    pub fn libusb_get_active_config_descriptor(dev: *const libusb_device, config: *mut *const libusb_config_descriptor) -> c_int;
    pub fn libusb_get_config_descriptor_by_value(dev: *const libusb_device, bConfigurationValue: u8, config: *mut *const libusb_config_descriptor) -> c_int;
    pub fn libusb_free_config_descriptor(config: *const libusb_config_descriptor);

    pub fn libusb_get_bus_number(dev: *const libusb_device) -> u8;
    pub fn libusb_get_port_number(dev: *mut libusb_device) -> u8;
    pub fn libusb_get_port_numbers(dev: *mut libusb_device, port_numbers: *mut u8, port_numbers_len: c_int) -> c_int;
    pub fn libusb_get_port_path(context: *mut libusb_context, dev: *mut libusb_device, path: *mut u8, path_length: u8) -> c_int;
    pub fn libusb_get_device_address(dev: *const libusb_device) -> u8;
    pub fn libusb_get_device_speed(dev: *const libusb_device) -> c_int;
    pub fn libusb_get_max_packet_size(dev: *const libusb_device, endpoint: c_uchar) -> c_int;
    pub fn libusb_get_max_iso_packet_size(dev: *const libusb_device, endpoint: c_uchar) -> c_int;

    pub fn libusb_open(dev: *const libusb_device, handle: *mut *mut libusb_device_handle) -> c_int;
    pub fn libusb_close(dev_handle: *mut libusb_device_handle);
    pub fn libusb_open_device_with_vid_pid(context: *mut libusb_context, vendor_id: u16, product_id: u16) -> *mut libusb_device_handle;
    pub fn libusb_reset_device(dev_handle: *mut libusb_device_handle) -> c_int;
    pub fn libusb_clear_halt(dev_handle: *mut libusb_device_handle, endpoint: c_uchar) -> c_int;
    pub fn libusb_alloc_streams(dev_handle: *mut libusb_device_handle, num_streams: u32, endpoints: *mut c_uchar, num_endpoints: c_int) -> c_int;
    pub fn libusb_free_streams(dev_handle: *mut libusb_device_handle, endpoints: *mut c_uchar, num_endpoints: c_int) -> c_int;
    pub fn libusb_get_string_descriptor_ascii(dev_handle: *mut libusb_device_handle, desc_index: u8, data: *mut c_uchar, length: c_int) -> c_int;

    pub fn libusb_get_configuration(dev_handle: *mut libusb_device_handle, config: *mut c_int) -> c_int;
    pub fn libusb_set_configuration(dev_handle: *mut libusb_device_handle, config: c_int) -> c_int;

    pub fn libusb_get_ss_endpoint_companion_descriptor(context: *mut libusb_context, endpoint: *const libusb_endpoint_descriptor, ep_comp: *mut *const libusb_ss_endpoint_companion_descriptor) -> c_int;
    pub fn libusb_free_ss_endpoint_companion_descriptor(ep_comp: *mut libusb_ss_endpoint_companion_descriptor);
    pub fn libusb_get_bos_descriptor(dev_handle: *mut libusb_device_handle, bos: *mut *const libusb_bos_descriptor) -> c_int;
    pub fn libusb_free_bos_descriptor(bos: *mut libusb_bos_descriptor);
    pub fn libusb_get_usb_2_0_extension_descriptor(context: *mut libusb_context, dev_cap: *mut libusb_bos_dev_capability_descriptor, usb_2_0_extension: *mut *const libusb_usb_2_0_extension_descriptor) -> c_int;
    pub fn libusb_free_usb_2_0_extension_descriptor(usb_2_0_extension: *mut libusb_usb_2_0_extension_descriptor);
    pub fn libusb_get_ss_usb_device_capability_descriptor(context: *mut libusb_context, dev_cap: *mut libusb_bos_dev_capability_descriptor, ss_usb_device_cap: *mut *const libusb_ss_usb_device_capability_descriptor) -> c_int;
    pub fn libusb_free_ss_usb_device_capability_descriptor(ss_usb_device_cap: *mut libusb_ss_usb_device_capability_descriptor);
    pub fn libusb_get_container_id_descriptor(context: *mut libusb_context, dev_cap: *mut libusb_bos_dev_capability_descriptor, container_id: *mut *const libusb_container_id_descriptor) -> c_int;
    pub fn libusb_free_container_id_descriptor(container_id: *mut libusb_container_id_descriptor);

    pub fn libusb_set_auto_detach_kernel_driver(dev_handle: *mut libusb_device_handle, enable: c_int) -> c_int;
    pub fn libusb_kernel_driver_active(dev_handle: *mut libusb_device_handle, interface_number: c_int) -> c_int;
    pub fn libusb_detach_kernel_driver(dev_handle: *mut libusb_device_handle, interface_number: c_int) -> c_int;
    pub fn libusb_attach_kernel_driver(dev_handle: *mut libusb_device_handle, interface_number: c_int) -> c_int;

    pub fn libusb_claim_interface(dev_handle: *mut libusb_device_handle, interface_number: c_int) -> c_int;
    pub fn libusb_release_interface(dev_handle: *mut libusb_device_handle, interface_number: c_int) -> c_int;
    pub fn libusb_set_interface_alt_setting(dev_handle: *mut libusb_device_handle, interface_number: c_int, alternate_setting: c_int) -> c_int;

    pub fn libusb_interrupt_transfer(dev_handle: *mut libusb_device_handle, endpoint: c_uchar, data: *mut c_uchar, length: c_int, transferred: *mut c_int, timeout: c_uint) -> c_int;
    pub fn libusb_bulk_transfer(dev_handle: *mut libusb_device_handle, endpoint: c_uchar, data: *mut c_uchar, length: c_int, transferred: *mut c_int, timeout: c_uint) -> c_int;
    pub fn libusb_control_transfer(dev_handle: *mut libusb_device_handle, request_type: u8, request: u8, value: u16, index: u16, data: *mut c_uchar, length: u16, timeout: c_uint) -> c_int;

    pub fn libusb_alloc_transfer(iso_packets: c_int) -> *mut libusb_transfer;
    pub fn libusb_submit_transfer(transfer: *mut libusb_transfer) -> c_int;
    pub fn libusb_cancel_transfer(transfer: *mut libusb_transfer) -> c_int;
    pub fn libusb_free_transfer(transfer: *mut libusb_transfer);
    pub fn libusb_transfer_set_stream_id(transfer: *mut libusb_transfer, stream_id: u32);
    pub fn libusb_transfer_get_stream_id(transfer: *mut libusb_transfer) -> u32;

    pub fn libusb_handle_events(context: *mut libusb_context) -> c_int;
    pub fn libusb_handle_events_timeout(context: *mut libusb_context, tv: *const timeval) -> c_int;
    pub fn libusb_handle_events_completed(context: *mut libusb_context, completed: *mut c_int) -> c_int;
    pub fn libusb_handle_events_timeout_completed(context: *mut libusb_context, tv: *const timeval, completed: *mut c_int) -> c_int;
    pub fn libusb_handle_events_locked(context: *mut libusb_context, tv: *const timeval) -> c_int;

    pub fn libusb_try_lock_events(context: *mut libusb_context) -> c_int;
    pub fn libusb_lock_events(context: *mut libusb_context);
    pub fn libusb_unlock_events(context: *mut libusb_context);
    pub fn libusb_event_handling_ok(context: *mut libusb_context) -> c_int;
    pub fn libusb_event_handler_active(context: *mut libusb_context) -> c_int;
    pub fn libusb_lock_event_waiters(context: *mut libusb_context);
    pub fn libusb_unlock_event_waiters(context: *mut libusb_context);
    pub fn libusb_wait_for_event(context: *mut libusb_context, tv: *const timeval) -> c_int;

    pub fn libusb_pollfds_handle_timeouts(context: *mut libusb_context) -> c_int;
    pub fn libusb_get_next_timeout(context: *mut libusb_context, tv: *mut timeval) -> c_int;
    pub fn libusb_get_pollfds(context: *mut libusb_context) -> *const *mut libusb_pollfd;
    pub fn libusb_set_pollfd_notifiers(context: *mut libusb_context, added_cb: libusb_pollfd_added_cb, removed_cb: libusb_pollfd_removed_cb, user_data: *mut c_void);

    pub fn libusb_hotplug_register_callback(context: *mut libusb_context, events: c_int, flags: c_int, vendor_id: c_int, product_id: c_int, dev_class: c_int, cb_fn: libusb_hotplug_callback_fn, user_data: *mut c_void, callback_handle: *mut libusb_hotplug_callback_handle) -> c_int;
    pub fn libusb_hotplug_deregister_callback(context: *mut libusb_context, callback_handle: libusb_hotplug_callback_handle);
    pub fn libusb_hotplug_get_user_data(context: *mut libusb_context, callback_handle: libusb_hotplug_callback_handle) -> *mut c_void;
}


// defined as static inline in libusb.h
pub unsafe fn libusb_get_string_descriptor(dev_handle: *mut libusb_device_handle,
                                           desc_index: u8,
                                           langid: u16,
                                           data: *mut c_uchar,
                                           length: c_int) -> c_int {
    libusb_control_transfer(dev_handle,
                            LIBUSB_ENDPOINT_IN,
                            LIBUSB_REQUEST_GET_DESCRIPTOR,
                            (LIBUSB_DT_STRING as u16) << 8 | desc_index as u16,
                            langid, data,
                            length as u16,
                            1000)
}

#[repr(C)]
union ToLe16 {
    b8: [u8;2],
    b16: u16
}

/// Convert a 16-bit value from host-endian to little-endian format. On
/// little endian systems, this function does nothing. On big endian systems,
/// the bytes are swapped.
/// x the host-endian value to convert
/// returns the value in little-endian byte order
pub fn libusb_cpu_to_le16(x: u16) -> u16 {
    let _temp = ToLe16 {
        b8: [(x >> 8) as u8,(x & 0xff) as u8]
    };
    unsafe { _temp.b16 }
}

/// Get the data section of a control transfer. This convenience function is here
/// to remind you that the data does not start until 8 bytes into the actual
/// buffer, as the setup packet comes first.
///
/// Calling this function only makes sense from a transfer callback function,
/// or situations where you have already allocated a suitably sized buffer at
/// transfer->buffer.
///
/// param transfer a transfer
/// returns pointer to the first byte of the data section
pub unsafe fn libusb_control_transfer_get_data(transfer: *mut libusb_transfer) -> *mut c_uchar {
    (*transfer).buffer.offset(LIBUSB_CONTROL_SETUP_SIZE as isize)
}

#[allow(non_snake_case)]
/// Helper function to populate the setup packet (first 8 bytes of the data
/// buffer) for a control transfer. The wIndex, wValue and wLength values should
/// be given in host-endian byte order.
pub unsafe fn libusb_fill_control_setup(buffer: *mut c_uchar,
                                        bmRequestType: u8,
                                        bRequest: u8,
                                        wValue: u16,
                                        wIndex: u16,
                                        wLength: u16) {
    let setup: &mut libusb_control_setup = std::mem::transmute(buffer);
    setup.bmRequestType = bmRequestType;
    setup.bRequest = bRequest;
    setup.wValue = libusb_cpu_to_le16(wValue);
    setup.wIndex = libusb_cpu_to_le16(wIndex);
    setup.wLength = libusb_cpu_to_le16(wLength);
}

/// Helper function to populate the required \ref libusb_transfer fields
/// for a control transfer.
///
/// If you pass a transfer buffer to this function, the first 8 bytes will
/// be interpreted as a control setup packet, and the wLength field will be
/// used to automatically populate the \ref libusb_transfer::length "length"
/// field of the transfer. Therefore the recommended approach is:
/// -# Allocate a suitably sized data buffer (including space for control setup)
/// -# Call libusb_fill_control_setup()
/// -# If this is a host-to-device transfer with a data stage, put the data
///   in place after the setup packet
/// -# Call this function
/// -# Call libusb_submit_transfer()
///
/// It is also legal to pass a NULL buffer to this function, in which case this
/// function will not attempt to populate the length field. Remember that you
/// must then populate the buffer and length fields later.
pub unsafe fn libusb_fill_control_transfer(transfer: *mut libusb_transfer,
                                           dev_handle: *mut libusb_device_handle,
                                           buffer: *mut c_uchar,
                                           callback: libusb_transfer_cb_fn,
                                           user_data: *mut c_void,
                                           timeout: c_uint) {
    // struct libusb_control_setup *setup = (struct libusb_control_setup *)(void *) buffer;
    let setup: *mut libusb_control_setup = std::mem::transmute(buffer);
    (*transfer).dev_handle = dev_handle;
    (*transfer).endpoint = 0;
    (*transfer).transfer_type = LIBUSB_TRANSFER_TYPE_CONTROL;
    (*transfer).timeout = timeout;
    (*transfer).buffer = buffer;
    let ptr = setup as *const usize;
    if !ptr.is_null()  {
        // let size_of_result = libusb_cpu_to_le16((*setup).wLength);
        (*transfer).length = (LIBUSB_CONTROL_SETUP_SIZE + std::mem::size_of::<u16>()).try_into().unwrap();
    }
    (*transfer).user_data = user_data;
    (*transfer).callback = callback;
}

/// Helper function to populate the required \ref libusb_transfer fields
/// for a bulk transfer.
///
/// param transfer the transfer to populate
/// param dev_handle handle of the device that will handle the transfer
/// param endpoint address of the endpoint where this transfer will be sent
/// param buffer data buffer
/// param length length of data buffer
/// param callback callback function to be invoked on transfer completion
/// param user_data user data to pass to callback function
/// param timeout timeout for the transfer in milliseconds
pub unsafe fn libusb_fill_bulk_transfer(transfer: *mut libusb_transfer,
                                        dev_handle: *mut libusb_device_handle,
                                        endpoint: c_uchar,
                                        buffer: *mut c_uchar,
                                        length: c_int,
                                        callback: libusb_transfer_cb_fn,
                                        user_data: *mut c_void,
                                        timeout: c_uint) {
    (*transfer).dev_handle = dev_handle;
    (*transfer).endpoint = endpoint;
    (*transfer).transfer_type = LIBUSB_TRANSFER_TYPE_BULK;
    (*transfer).timeout = timeout;
    (*transfer).buffer = buffer;
    (*transfer).length = length;
    (*transfer).user_data = user_data;
    (*transfer).callback = callback;
}

/// Helper function to populate the required \ref libusb_transfer fields
/// for a bulk transfer using bulk streams.
///
/// Since version 1.0.19, \ref LIBUSB_API_VERSION >= 0x01000103
///
/// param transfer the transfer to populate
/// param dev_handle handle of the device that will handle the transfer
/// param endpoint address of the endpoint where this transfer will be sent
/// param stream_id bulk stream id for this transfer
/// param buffer data buffer
/// param length length of data buffer
/// param callback callback function to be invoked on transfer completion
/// param user_data user data to pass to callback function
/// param timeout timeout for the transfer in milliseconds
pub unsafe fn libusb_fill_bulk_stream_transfer(transfer: *mut libusb_transfer,
                                               dev_handle: *mut libusb_device_handle,
                                               endpoint: c_uchar,
                                               stream_id: u32,
                                               buffer: *mut c_uchar,
                                               length: c_int,
                                               callback: libusb_transfer_cb_fn,
                                               user_data: *mut c_void,
                                               timeout: c_uint) {
    libusb_fill_bulk_transfer(transfer, dev_handle, endpoint, buffer,
                              length, callback, user_data, timeout);
    (*transfer).transfer_type = LIBUSB_TRANSFER_TYPE_BULK_STREAM;
    libusb_transfer_set_stream_id(transfer, stream_id);
}

/// Helper function to populate the required \ref libusb_transfer fields
/// for an interrupt transfer.
///
/// param transfer the transfer to populate
/// param dev_handle handle of the device that will handle the transfer
/// param endpoint address of the endpoint where this transfer will be sent
/// param buffer data buffer
/// param length length of data buffer
/// param callback callback function to be invoked on transfer completion
/// param user_data user data to pass to callback function
/// param timeout timeout for the transfer in milliseconds
pub unsafe fn libusb_fill_interrupt_transfer(transfer: *mut libusb_transfer,
                                             dev_handle: *mut libusb_device_handle,
                                             endpoint: c_uchar,
                                             buffer: *mut c_uchar,
                                             length: c_int,
                                             callback: libusb_transfer_cb_fn,
                                             user_data: *mut c_void,
                                             timeout: c_uint) {
    (*transfer).dev_handle = dev_handle;
    (*transfer).endpoint = endpoint;
    (*transfer).transfer_type = LIBUSB_TRANSFER_TYPE_INTERRUPT;
    (*transfer).timeout = timeout;
    (*transfer).buffer = buffer;
    (*transfer).length = length;
    (*transfer).user_data = user_data;
    (*transfer).callback = callback;
}

pub unsafe fn libusb_fill_iso_transfer(transfer: *mut libusb_transfer,
                                       dev_handle: *mut libusb_device_handle,
                                       endpoint: c_uchar,
                                       buffer: *mut c_uchar,
                                       length: c_int,
                                       num_iso_packets: c_int,
                                       callback: libusb_transfer_cb_fn,
                                       user_data: *mut c_void,
                                       timeout: c_uint) {
    (*transfer).dev_handle = dev_handle;
    (*transfer).endpoint = endpoint;
    (*transfer).transfer_type = LIBUSB_TRANSFER_TYPE_ISOCHRONOUS;
    (*transfer).timeout = timeout;
    (*transfer).buffer = buffer;
    (*transfer).length = length;
    (*transfer).num_iso_packets = num_iso_packets;
    (*transfer).user_data = user_data;
    (*transfer).callback = callback;
}

pub unsafe fn libusb_set_iso_packet_lengths(transfer: *mut libusb_transfer,
                                            length: c_uint) {
    for one_descriptor in (*transfer).iso_packet_desc.iter_mut() {
        one_descriptor.length = length;
    }
}
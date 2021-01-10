extern crate libusb_sys as ffi;
extern crate libc;

use libc::c_int;

use std::mem;

fn main() {
    let mut context: *mut ::ffi::libusb_context = unsafe { mem::uninitialized() };

    match unsafe { ::ffi::libusb_init(&mut context) } {
        0 => (),
        e => panic!("libusb_init: {}", get_error(e))
    };

    listen_for_events(context)
        .expect("Couldn't setup listener");

    while unsafe { ::ffi::libusb_handle_events(context) } == 0 {}

    unsafe { ::ffi::libusb_exit(context) };
}

fn listen_for_events(context: *mut ::ffi::libusb_context) -> Result<(), String> {
    if ! unsafe { ::ffi::libusb_has_capability(::ffi::LIBUSB_CAP_HAS_HOTPLUG) > 0 } {
        return Err("Libusb doesn't support hotplug on this platform".into());
    }

    let res = unsafe { ::ffi::libusb_hotplug_register_callback(
        context,
        ::ffi::LIBUSB_HOTPLUG_EVENT_DEVICE_ARRIVED | ::ffi::LIBUSB_HOTPLUG_EVENT_DEVICE_LEFT,
        ::ffi::LIBUSB_HOTPLUG_ENUMERATE,
        ::ffi::LIBUSB_HOTPLUG_MATCH_ANY,
        ::ffi::LIBUSB_HOTPLUG_MATCH_ANY,
        ::ffi::LIBUSB_HOTPLUG_MATCH_ANY,
        callback_fn,
        0 as *mut ::std::ffi::c_void,
        0 as *mut i32) };
    if res != ::ffi::LIBUSB_SUCCESS {
        return Err("Failed to setup callback".into());
    }

    return Ok(())
}

extern "C" fn callback_fn(_ctx: *mut ::ffi::libusb_context, _device: *const ::ffi::libusb_device, event: i32, _data: *mut ::std::ffi::c_void) -> i32 {
    match event {
        e if e == ::ffi::LIBUSB_HOTPLUG_EVENT_DEVICE_ARRIVED => println!("Device attached!"),
        e if e == ::ffi::LIBUSB_HOTPLUG_EVENT_DEVICE_LEFT =>  println!("Device removed!"),
        e => println!("Some other event arrived?: {}", e),
    };
    0
}

fn get_error(err: c_int) -> &'static str {
    match err {
        ::ffi::LIBUSB_SUCCESS             => "success",
        ::ffi::LIBUSB_ERROR_IO            => "I/O error",
        ::ffi::LIBUSB_ERROR_INVALID_PARAM => "invalid parameter",
        ::ffi::LIBUSB_ERROR_ACCESS        => "access denied",
        ::ffi::LIBUSB_ERROR_NO_DEVICE     => "no such device",
        ::ffi::LIBUSB_ERROR_NOT_FOUND     => "entity not found",
        ::ffi::LIBUSB_ERROR_BUSY          => "resource busy",
        ::ffi::LIBUSB_ERROR_TIMEOUT       => "opteration timed out",
        ::ffi::LIBUSB_ERROR_OVERFLOW      => "overflow error",
        ::ffi::LIBUSB_ERROR_PIPE          => "pipe error",
        ::ffi::LIBUSB_ERROR_INTERRUPTED   => "system call interrupted",
        ::ffi::LIBUSB_ERROR_NO_MEM        => "insufficient memory",
        ::ffi::LIBUSB_ERROR_NOT_SUPPORTED => "operation not supported",
        ::ffi::LIBUSB_ERROR_OTHER | _     => "other error"
    }
}


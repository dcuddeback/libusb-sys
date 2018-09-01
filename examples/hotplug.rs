extern crate libusb_sys as ffi;
extern crate libc;

use libc::{c_int, c_void, timeval};

use std::mem;
use std::ptr;
use std::time::Instant;

fn main() {
    let mut context: *mut ::ffi::libusb_context = unsafe { mem::uninitialized() };

    match unsafe { ::ffi::libusb_init(&mut context) } {
        0 => (),
        e => panic!("libusb_init: {}", get_error(e)),
    };

    if unsafe { ::ffi::libusb_has_capability(::ffi::LIBUSB_CAP_HAS_HOTPLUG) == 0 } {
        panic!("libusb doesn't support hotplug on this platform");
    }

    let mut callback_handle = unsafe { mem::uninitialized() };

    let res = unsafe {
        ::ffi::libusb_hotplug_register_callback(
            context,
            ::ffi::LIBUSB_HOTPLUG_EVENT_DEVICE_ARRIVED | ::ffi::LIBUSB_HOTPLUG_EVENT_DEVICE_LEFT,
            ::ffi::LIBUSB_HOTPLUG_ENUMERATE,
            ::ffi::LIBUSB_HOTPLUG_MATCH_ANY,
            ::ffi::LIBUSB_HOTPLUG_MATCH_ANY,
            ::ffi::LIBUSB_HOTPLUG_MATCH_ANY,
            callback_fn,
            ptr::null_mut() as *mut c_void,
            &mut callback_handle as *mut ::ffi::libusb_hotplug_callback_handle,
        )
    };

    if res != ::ffi::LIBUSB_SUCCESS {
        panic!("libusb_hotplug_register_callback: {}", get_error(res));
    }

    let start = Instant::now();
    let mut timeout = timeval {
        tv_sec: 0,
        tv_usec: 100_000,
    };

    while start.elapsed().as_secs() < 30 {
        match unsafe { ::ffi::libusb_handle_events_timeout_completed(context, &mut timeout, ptr::null_mut()) } {
            0 => (),
            e => panic!("libusb_handle_events: {}", get_error(e)),
        };
    }

    unsafe {
        ::ffi::libusb_hotplug_deregister_callback(context, callback_handle);
        ::ffi::libusb_exit(context);
    }
}

extern "C" fn callback_fn(_ctx: *mut ::ffi::libusb_context, device: *mut ::ffi::libusb_device, event: ::ffi::libusb_hotplug_event, _data: *mut c_void) -> i32 {
    let event_name = match event {
        ::ffi::LIBUSB_HOTPLUG_EVENT_DEVICE_ARRIVED => "ATTACH",
        ::ffi::LIBUSB_HOTPLUG_EVENT_DEVICE_LEFT =>  "REMOVE",
        _ => "OTHER",
    };

    let mut device_descriptor = unsafe { mem::uninitialized() };

    unsafe {
        match ::ffi::libusb_get_device_descriptor(device, &mut device_descriptor) {
            0 => {
                println!(
                    "{}: {:04X}:{:04X} {}.{}.{}",
                     event_name,
                     device_descriptor.idVendor,
                     device_descriptor.idProduct,
                     ::ffi::libusb_get_bus_number(device),
                     ::ffi::libusb_get_port_number(device),
                     ::ffi::libusb_get_device_address(device),
                );
            },
            e => println!("libusb_get_device_descriptor: {}", get_error(e)),
        };
    }

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

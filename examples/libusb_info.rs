extern crate libusb_sys as ffi;
extern crate libc;

use std::mem;
use std::str;
use std::ffi::CStr;
use std::mem::MaybeUninit;
use libc::{c_char, c_int};

#[link(name = "libusb-1.0")]
extern "C" fn call_libusb_log_cb(context: *mut ::ffi::libusb_context, log_level: c_int, log_message: *const c_char) {
  if !context.is_null() {
    println!("{} : {:?}", log_level, &log_message);
  } else {
    println!("No USB context");
  }
}

fn main() {
  print_version();
  print_capabilities();
}

fn print_version() {
  let version: &ffi::libusb_version = unsafe {
      mem::transmute(::ffi::libusb_get_version())
  };

  let rc       = str::from_utf8(unsafe { CStr::from_ptr(version.rc)       }.to_bytes()).unwrap_or("");
  let describe = str::from_utf8(unsafe { CStr::from_ptr(version.describe) }.to_bytes()).unwrap_or("");

  println!("libusb v{}.{}.{}.{}{} {}", version.major, version.minor, version.micro, version.nano, rc, describe);
}

fn print_capabilities() {
  let mut context_uninit: MaybeUninit<*mut ::ffi::libusb_context> = MaybeUninit::uninit();

  // library must be initialized before calling libusb_has_capabililty()
  match unsafe { ::ffi::libusb_init(context_uninit.as_mut_ptr()) } {
    0 => (),
    e => panic!("libusb_init: {}", e)
  };

  let context = unsafe { context_uninit.assume_init() };
  unsafe {
    ::ffi::libusb_set_debug(context, ::ffi::LIBUSB_LOG_LEVEL_DEBUG);
    ::ffi::libusb_set_debug(context, ::ffi::LIBUSB_LOG_LEVEL_INFO);
    ::ffi::libusb_set_debug(context, ::ffi::LIBUSB_LOG_LEVEL_WARNING);
    ::ffi::libusb_set_debug(context, ::ffi::LIBUSB_LOG_LEVEL_ERROR);
    ::ffi::libusb_set_debug(context, ::ffi::LIBUSB_LOG_LEVEL_NONE);
  }

  unsafe { ::ffi::libusb_set_log_cb(context, call_libusb_log_cb,::ffi::LIBUSB_LOG_LEVEL_DEBUG) };

  println!("has capability? {}", unsafe { ::ffi::libusb_has_capability(::ffi::LIBUSB_CAP_HAS_CAPABILITY) });
  println!("has hotplug? {}", unsafe { ::ffi::libusb_has_capability(::ffi::LIBUSB_CAP_HAS_HOTPLUG) });
  println!("has HID access? {}", unsafe { ::ffi::libusb_has_capability(::ffi::LIBUSB_CAP_HAS_HID_ACCESS) });
  println!("supports detach kernel driver? {}", unsafe { ::ffi::libusb_has_capability(::ffi::LIBUSB_CAP_SUPPORTS_DETACH_KERNEL_DRIVER) });

  unsafe { ::ffi::libusb_exit(context) };
}

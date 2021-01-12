extern crate libusb_sys as ffi;
extern crate libc;

use libc::{c_int, c_uchar, c_char};

use std::{mem::{MaybeUninit}, slice, ptr};

#[link(name = "libusb-1.0")]
extern "C" fn call_libusb_log_cb(_context: *mut ::ffi::libusb_context, log_level: c_int, log_message: *const c_char) {
    println!("USB_CallBack - {:?} : {:?}", log_level, log_message);
}

fn main() {
  let mut context_uninit: MaybeUninit<*mut ::ffi::libusb_context> = MaybeUninit::uninit();

  match unsafe { ::ffi::libusb_init(context_uninit.as_mut_ptr()) } {
    0 => (),
    e => panic!("libusb_init: {}", get_error(e))
  };
  let context = unsafe { context_uninit.assume_init() };
  unsafe {
    // ::ffi::libusb_set_debug(context, ::ffi::LIBUSB_LOG_LEVEL_DEBUG);
    // ::ffi::libusb_set_debug(context, ::ffi::LIBUSB_LOG_LEVEL_INFO);
    // ::ffi::libusb_set_debug(context, ::ffi::LIBUSB_LOG_LEVEL_WARNING);
    ::ffi::libusb_set_debug(context, ::ffi::LIBUSB_LOG_LEVEL_ERROR);
  }
  unsafe { ::ffi::libusb_set_log_cb(context, call_libusb_log_cb,::ffi::LIBUSB_LOG_LEVEL_DEBUG) };
  list_devices(context);

  unsafe { ::ffi::libusb_exit(context) };
}

fn list_devices(context: *mut ::ffi::libusb_context) {
  let mut device_list_uninit: MaybeUninit<*const *mut ffi::libusb_device> = MaybeUninit::uninit();

  let get_device_list_result = unsafe { ::ffi::libusb_get_device_list(context, device_list_uninit.as_mut_ptr()) };

  if get_device_list_result < 0 {
    println!("libusb_get_device_list: {}", get_error(get_device_list_result as c_int));
    return;
  }
  let device_list: *const *mut ffi::libusb_device = unsafe { device_list_uninit.assume_init() };
  let devs = unsafe { slice::from_raw_parts(device_list, get_device_list_result as usize) };

  for dev in devs {
    display_device(dev);
  }

  unsafe { ::ffi::libusb_free_device_list(device_list, 1) };
}

fn display_device(dev: &*mut ::ffi::libusb_device) {
  let mut descriptor_uninit: MaybeUninit<::ffi::libusb_device_descriptor> = MaybeUninit::uninit();
  let mut handle: *mut ::ffi::libusb_device_handle = ptr::null_mut();

  let bus = unsafe { ::ffi::libusb_get_bus_number(*dev) };
  let address = unsafe { ::ffi::libusb_get_device_address(*dev) };
  let speed = unsafe { ::ffi::libusb_get_device_speed(*dev) };

  let has_descriptor = match unsafe { ::ffi::libusb_get_device_descriptor(*dev, descriptor_uninit.as_mut_ptr()) } {
    0 => true,
    _ => false
  };
  let descriptor = unsafe { descriptor_uninit.assume_init() };
  if unsafe { ::ffi::libusb_open(*dev, &mut handle) } < 0 {
    println!("Couldn't open device, some information will be missing"); // luck of OS permissions usually
    handle = ptr::null_mut();
  }

  print!("Bus {:03} Device {:03}", bus, address);

  if has_descriptor {
    print!(" ID {:04x}:{:04x}", descriptor.idVendor, descriptor.idProduct);
  }

  print!(", Speed : {}", get_device_speed(speed));

  if has_descriptor {
    if descriptor.iManufacturer > 0 {
      match get_string_descriptor(handle, descriptor.iManufacturer) {
        Some(s) => print!(" {}", s),
        None => ()
      }
    }

    if descriptor.iProduct > 0 {
      match get_string_descriptor(handle, descriptor.iProduct) {
        Some(s) => print!(" {}", s),
        None => ()
      }
    }

    if descriptor.iSerialNumber > 0 {
      match get_string_descriptor(handle, descriptor.iSerialNumber) {
        Some(s) => print!(" Serial No. {}", s),
        None => ()
      }
    }
  }

  println!("\n--------------------------------------------------");

  if has_descriptor {
    print_device_descriptor(handle, &descriptor);

    for i in 0..descriptor.bNumConfigurations {
      let mut descriptor_uninit: MaybeUninit<*const ::ffi::libusb_config_descriptor> = MaybeUninit::uninit();

      match unsafe { ::ffi::libusb_get_config_descriptor(*dev, i, descriptor_uninit.as_mut_ptr()) } {
        0 => {
          let descriptor = unsafe { descriptor_uninit.assume_init() };
          let config = unsafe { &*descriptor };
          let interfaces = unsafe { slice::from_raw_parts(config.interface, config.bNumInterfaces as usize) };

          print_config_descriptor(handle, config);

          for iface in interfaces {
            let iface_descriptors = unsafe { slice::from_raw_parts(iface.altsetting, iface.num_altsetting as usize) };

            for iface_desc in iface_descriptors {
              print_interface_descriptor(handle, iface_desc);

              let endpoints = unsafe { slice::from_raw_parts(iface_desc.endpoint, iface_desc.bNumEndpoints as usize) };

              for endpoint in endpoints {
                print_endpoint_descriptor(endpoint);
              }
            }
          }

          unsafe { ::ffi::libusb_free_config_descriptor(descriptor) };
        },
        _ => ()
      }
    }
  }

  if !handle.is_null() {
    unsafe { ::ffi::libusb_close(handle) };
  }
}

fn print_device_descriptor(handle: *mut ::ffi::libusb_device_handle, descriptor: &::ffi::libusb_device_descriptor) {
  println!("Device Descriptor:");
  println!("  bLength: {:16}", descriptor.bLength);
  println!("  bDescriptorType: {:8} {}", descriptor.bDescriptorType, get_descriptor_type(descriptor.bDescriptorType));
  println!("  bcdUSB:            {:#06x} {}", descriptor.bcdUSB, get_bcd_version(descriptor.bcdUSB));
  println!("  bDeviceClass:        {:#04x} {}", descriptor.bDeviceClass, get_class_type(descriptor.bDeviceClass));
  println!("  bDeviceSubClass: {:8}", descriptor.bDeviceSubClass);
  println!("  bDeviceProtocol: {:8}", descriptor.bDeviceProtocol);
  println!("  bMaxPacketSize0: {:8}", descriptor.bMaxPacketSize0);
  println!("  idVendor:          {:#06x}", descriptor.idVendor);
  println!("  idProduct:         {:#06x}", descriptor.idProduct);
  println!("  bcdDevice:         {:#06x}", descriptor.bcdDevice);
  println!("  iManufacturer: {:10} {}", descriptor.iManufacturer, get_string_descriptor(handle, descriptor.iManufacturer).unwrap_or(String::new()));
  println!("  iProduct: {:15} {}", descriptor.iProduct, get_string_descriptor(handle, descriptor.iProduct).unwrap_or(String::new()));
  println!("  iSerialNumber: {:10} {}", descriptor.iSerialNumber, get_string_descriptor(handle, descriptor.iSerialNumber).unwrap_or(String::new()));
  println!("  bNumConfigurations: {:5}", descriptor.bNumConfigurations);
}

fn print_config_descriptor(handle: *mut ::ffi::libusb_device_handle, descriptor: &::ffi::libusb_config_descriptor) {
  println!("  Configuration Descriptor:");
  println!("    bLength: {:16}", descriptor.bLength);
  println!("    bDescriptorType: {:8} {}", descriptor.bDescriptorType, get_descriptor_type(descriptor.bDescriptorType));
  println!("    wTotalLength: {:11}", descriptor.wTotalLength);
  println!("    bNumInterfaces: {:9}", descriptor.bNumInterfaces);
  println!("    bConfigurationValue: {:4}", descriptor.bConfigurationValue);
  println!("    iConfiguration: {:9} {}", descriptor.iConfiguration, get_string_descriptor(handle, descriptor.iConfiguration).unwrap_or(String::new()));
  println!("    bmAttributes:        {:#04x}", descriptor.bmAttributes);
  println!("    bMaxPower: {:14} {}", descriptor.bMaxPower, get_max_power(descriptor.bMaxPower));

  if descriptor.extra_length > 0 {
    let extra = unsafe { slice::from_raw_parts(descriptor.extra, descriptor.extra_length as usize) };
    println!("    (extra: {:?})", extra);
  }
}

fn print_interface_descriptor(handle: *mut ::ffi::libusb_device_handle, descriptor: &::ffi::libusb_interface_descriptor) {
  println!("    Interface Descriptor:");
  println!("      bLength: {:16}", descriptor.bLength);
  println!("      bDescriptorType: {:8} {}", descriptor.bDescriptorType, get_descriptor_type(descriptor.bDescriptorType));
  println!("      bInterfaceNumber: {:7}", descriptor.bInterfaceNumber);
  println!("      bAlternateSetting: {:6}", descriptor.bAlternateSetting);
  println!("      bNumEndpoints: {:10}", descriptor.bNumEndpoints);
  println!("      bInterfaceClass:     {:#04x} {}", descriptor.bInterfaceClass, get_class_type(descriptor.bInterfaceClass));
  println!("      bInterfaceSubClass: {:5}", descriptor.bInterfaceSubClass);
  println!("      bInterfaceProtocol: {:5}", descriptor.bInterfaceProtocol);
  println!("      iInterface: {:13} {}", descriptor.iInterface, get_string_descriptor(handle, descriptor.iInterface).unwrap_or(String::new()));

  if descriptor.extra_length > 0 {
    let extra = unsafe { slice::from_raw_parts(descriptor.extra, descriptor.extra_length as usize) };
    println!("    (extra: {:?})", extra);
  }
}

fn print_endpoint_descriptor(descriptor: &::ffi::libusb_endpoint_descriptor) {
  println!("      Endpoint Descriptor:");
  println!("        bLength: {:16}", descriptor.bLength);
  println!("        bDescriptorType: {:8} {}", descriptor.bDescriptorType, get_descriptor_type(descriptor.bDescriptorType));
  println!("        bEndpointAddress:    {:#04x} {}", descriptor.bEndpointAddress, get_endpoint(descriptor.bEndpointAddress));
  println!("        bmAttributes:        {:#04x}", descriptor.bmAttributes);
  println!("          Transfer Type:           {}", get_transfer_type(descriptor.bmAttributes));
  println!("          Synch Type:              {}", get_synch_type(descriptor.bmAttributes));
  println!("          Usage Type:              {}", get_usage_type(descriptor.bmAttributes));
  println!("        wMaxPacketSize:    {:#06x}", descriptor.wMaxPacketSize);
  println!("        bInterval: {:14}", descriptor.bInterval);
  println!("        bRefresh: {:15}", descriptor.bRefresh);
  println!("        bSynchAddress: {:10}", descriptor.bSynchAddress);

  if descriptor.extra_length > 0 {
    let extra = unsafe { slice::from_raw_parts(descriptor.extra, descriptor.extra_length as usize) };
    println!("    (extra: {:?})", extra);
  }
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

fn get_device_speed(speed: c_int) -> &'static str {
  match speed {
    ::ffi::LIBUSB_SPEED_SUPER_PLUS  => "10000 Mbps",
    ::ffi::LIBUSB_SPEED_SUPER       => "5000 Mbps",
    ::ffi::LIBUSB_SPEED_HIGH        => " 480 Mbps",
    ::ffi::LIBUSB_SPEED_FULL        => "  12 Mbps",
    ::ffi::LIBUSB_SPEED_LOW         => " 1.5 Mbps",
    ::ffi::LIBUSB_SPEED_UNKNOWN | _ => "(unknown)"
  }
}

fn get_max_power(power: u8) -> String {
  if power > 0 {
    format!("{}mW", power as usize * 2)
  }
  else {
    String::new()
  }
}

fn get_descriptor_type(desc_type: u8) -> &'static str {
  match desc_type {
    ::ffi::LIBUSB_DT_DEVICE                => "Device",
    ::ffi::LIBUSB_DT_CONFIG                => "Configuration",
    ::ffi::LIBUSB_DT_STRING                => "String",
    ::ffi::LIBUSB_DT_INTERFACE             => "Interface",
    ::ffi::LIBUSB_DT_ENDPOINT              => "Endpoint",
    ::ffi::LIBUSB_DT_BOS                   => "BOS",
    ::ffi::LIBUSB_DT_DEVICE_CAPABILITY     => "Device Capability",
    ::ffi::LIBUSB_DT_HID                   => "HID",
    ::ffi::LIBUSB_DT_REPORT                => "Report",
    ::ffi::LIBUSB_DT_PHYSICAL              => "Physical",
    ::ffi::LIBUSB_DT_HUB                   => "HUB",
    ::ffi::LIBUSB_DT_SUPERSPEED_HUB        => "Superspeed Hub",
    ::ffi::LIBUSB_DT_SS_ENDPOINT_COMPANION => "Superspeed Endpoint Companion",
    _ => ""
  }
}

fn get_bcd_version(bcd_version: u16) -> String {
  let digit1 = (bcd_version & 0xF000) >> 12;
  let digit2 = (bcd_version & 0x0F00) >> 8;
  let digit3 = (bcd_version & 0x00F0) >> 4;
  let digit4 = (bcd_version & 0x000F) >> 0;

  if digit1 > 0 {
    format!("{}{}.{}{}", digit1, digit2, digit3, digit4)
  }
  else {
    format!("{}.{}{}", digit2, digit3, digit4)
  }
}

fn get_class_type(class: u8) -> &'static str {
  match class {
    ::ffi::LIBUSB_CLASS_PER_INTERFACE       => "(Defined at Interface level)",
    ::ffi::LIBUSB_CLASS_AUDIO               => "Audio",
    ::ffi::LIBUSB_CLASS_COMM                => "Comm",
    ::ffi::LIBUSB_CLASS_HID                 => "HID",
    ::ffi::LIBUSB_CLASS_PHYSICAL            => "Physical",
    ::ffi::LIBUSB_CLASS_PRINTER             => "Printer",
    ::ffi::LIBUSB_CLASS_IMAGE               => "Image",
    ::ffi::LIBUSB_CLASS_MASS_STORAGE        => "Mass Storage",
    ::ffi::LIBUSB_CLASS_HUB                 => "Hub",
    ::ffi::LIBUSB_CLASS_DATA                => "Data",
    ::ffi::LIBUSB_CLASS_SMART_CARD          => "Smart Card",
    ::ffi::LIBUSB_CLASS_CONTENT_SECURITY    => "Content Security",
    ::ffi::LIBUSB_CLASS_VIDEO               => "Video",
    ::ffi::LIBUSB_CLASS_PERSONAL_HEALTHCARE => "Personal Healthcare",
    ::ffi::LIBUSB_CLASS_DIAGNOSTIC_DEVICE   => "Diagnostic Device",
    ::ffi::LIBUSB_CLASS_WIRELESS            => "Wireless",
    ::ffi::LIBUSB_CLASS_APPLICATION         => "Application",
    ::ffi::LIBUSB_CLASS_VENDOR_SPEC         => "Vendor Specific",
    _ => ""
  }
}

fn get_endpoint(address: u8) -> String {
  let number = address & ::ffi::LIBUSB_ENDPOINT_ADDRESS_MASK;

  match address & ::ffi::LIBUSB_ENDPOINT_DIR_MASK {
    ::ffi::LIBUSB_ENDPOINT_IN      => format!("EP {} IN", number),
    ::ffi::LIBUSB_ENDPOINT_OUT | _ => format!("EP {} OUT", number)
  }
}

fn get_transfer_type(attributes: u8) -> &'static str {
  match attributes & ::ffi::LIBUSB_TRANSFER_TYPE_MASK {
    ::ffi::LIBUSB_TRANSFER_TYPE_CONTROL     => "Control",
    ::ffi::LIBUSB_TRANSFER_TYPE_ISOCHRONOUS => "Isochronous",
    ::ffi::LIBUSB_TRANSFER_TYPE_BULK        => "Bulk",
    ::ffi::LIBUSB_TRANSFER_TYPE_INTERRUPT   => "Interrupt",
    ::ffi::LIBUSB_TRANSFER_TYPE_BULK_STREAM => "Bulk Stream",
    _ => ""
  }
}

fn get_synch_type(attributes: u8) -> &'static str {
  match (attributes & ::ffi::LIBUSB_ISO_SYNC_TYPE_MASK) >> 2 {
    ::ffi::LIBUSB_ISO_SYNC_TYPE_NONE     => "None",
    ::ffi::LIBUSB_ISO_SYNC_TYPE_ASYNC    => "Async",
    ::ffi::LIBUSB_ISO_SYNC_TYPE_ADAPTIVE => "Adaptive",
    ::ffi::LIBUSB_ISO_SYNC_TYPE_SYNC     => "Sync",
    _ => ""
  }
}

fn get_usage_type(attributes: u8) -> &'static str {
  match (attributes & ::ffi::LIBUSB_ISO_USAGE_TYPE_MASK) >> 4 {
    ::ffi::LIBUSB_ISO_USAGE_TYPE_DATA     => "Data",
    ::ffi::LIBUSB_ISO_USAGE_TYPE_FEEDBACK => "Feedback",
    ::ffi::LIBUSB_ISO_USAGE_TYPE_IMPLICIT => "Implicit",
    _ => ""
  }
}

fn get_string_descriptor(handle: *mut ::ffi::libusb_device_handle, desc_index: u8) -> Option<String> {
  if handle.is_null() || desc_index == 0{
    return None
  }

  let mut vec = Vec::<u8>::with_capacity(256);
  let ptr = (&mut vec[..]).as_mut_ptr();

  let len = unsafe { ::ffi::libusb_get_string_descriptor_ascii(handle, desc_index, ptr as *mut c_uchar, vec.capacity() as c_int) };

  if len > 0 {
    unsafe { vec.set_len(len as usize) };

    match String::from_utf8(vec) {
      Ok(s) => Some(s),
      Err(_) => None
    }
  }
  else {
    None
  }
}

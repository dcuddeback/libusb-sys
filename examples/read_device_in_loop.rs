extern crate libusb_sys as ffi;
extern crate libc;

use libc::{c_int,c_uint,c_uchar};

use std::slice;

use std::io::{Read,Cursor};
use std::str::FromStr;
use std::mem::MaybeUninit;
use std::ptr;

#[derive(Debug)]
struct Endpoint {
  config: u8,
  iface: u8,
  setting: u8,
  address: u8
}

fn main() {
  let args: Vec<String> = std::env::args().collect();

  if args.len() < 2 {
    println!("usage: <vendor-id> <product-id>");
    return;
  }

  println!("try parse <vendor-id>: {}", args[1]);
  let vid: u16 = FromStr::from_str(args[1].as_ref()).unwrap();
  println!("try parse <product-id>: {}", args[2]);
  let pid: u16 = FromStr::from_str(args[2].as_ref()).unwrap();

  let mut context_uninit: MaybeUninit<*mut ffi::libusb_context> = MaybeUninit::uninit();
  let context: *mut ffi::libusb_context;
  match unsafe { ::ffi::libusb_init(context_uninit.as_mut_ptr()) } {
    0 => {
      context = unsafe { context_uninit.assume_init() }
    },
    e => panic!("libusb_init: {}", e)
  };
  // unsafe {
    // ::ffi::libusb_set_debug(context, ::ffi::LIBUSB_LOG_LEVEL_DEBUG);
    // ::ffi::libusb_set_debug(context, ::ffi::LIBUSB_LOG_LEVEL_INFO);
    // ::ffi::libusb_set_debug(context, ::ffi::LIBUSB_LOG_LEVEL_WARNING);
    // ::ffi::libusb_set_debug(context, ::ffi::LIBUSB_LOG_LEVEL_ERROR);
  // }

  let mut device_list_uninit: MaybeUninit<*const *mut ffi::libusb_device> = MaybeUninit::uninit();
  let get_device_list_result = unsafe { ::ffi::libusb_get_device_list(context, device_list_uninit.as_mut_ptr()) };
  if get_device_list_result < 0 {
    println!("libusb_get_device_list: {}", get_device_list_result);
    return;
  }
  let device_list: *const *mut ffi::libusb_device = unsafe { device_list_uninit.assume_init() };
  let devs = unsafe { slice::from_raw_parts(device_list, get_device_list_result as usize) };

  for device in devs {
    print_device_tree(*device);
    println!("");
    let mut handle: *mut ffi::libusb_device_handle = ptr::null_mut();
    if unsafe { ffi::libusb_open(device.cast(), &mut handle) } < 0 {
      println!("Couldn't open device [{:?}], some information will be missing", device);
      continue;
    }

    if !handle.is_null() {
      match unsafe { ::ffi::libusb_reset_device(handle) } {
        0 => {
          unsafe { ::ffi::libusb_set_auto_detach_kernel_driver(handle, 0) };

          let device = unsafe { ::ffi::libusb_get_device(handle) };
          unsafe { ::ffi::libusb_ref_device(device) };

          let languages = get_language_ids(handle);
          println!("Supported languages: {:?}", languages);

          let mut active_config_uninit = MaybeUninit::<c_int>::zeroed();
          match unsafe { ::ffi::libusb_get_configuration(handle, active_config_uninit.as_mut_ptr()) } {
            0 =>  {
              let active_config = unsafe { active_config_uninit.assume_init() };
              println!("Active configuration: {}", active_config)
            },
            e => println!("libusb_get_configuration: {}", e)
          }
          println!("");

          match find_readable_endpoint(device, ::ffi::LIBUSB_TRANSFER_TYPE_INTERRUPT, vid, pid) {
            Some(ep) => read_endpoint(handle, device, ep, ::ffi::LIBUSB_TRANSFER_TYPE_INTERRUPT),
            None => println!("No readable interrupt endpoint")
          }
          println!("");

          match find_readable_endpoint(device, ::ffi::LIBUSB_TRANSFER_TYPE_BULK, vid, pid) {
            Some(ep) => read_endpoint(handle, device, ep, ::ffi::LIBUSB_TRANSFER_TYPE_BULK),
            None => println!("No readable bulk endpoint")
          }

          unsafe { ::ffi::libusb_unref_device(device) };
        },
        e => println!("libusb_reset_device: {}", e)
      }
      unsafe { ::ffi::libusb_close(handle) };
    } else {
      println!("ERROR on opening by ffi::libusb_open(...)");
    }


  }
  unsafe { ::ffi::libusb_free_device_list(device_list, 1) };


  unsafe { ::ffi::libusb_exit(context) };
}

fn print_device_tree(device: *mut ::ffi::libusb_device) -> usize {
  if device.is_null() {
    return 0;
  }

  let parent = unsafe { ::ffi::libusb_get_parent(device) };
  let depth = print_device_tree(parent);

  for _ in 0..depth {
    print!("  ");
  }

  let bus = unsafe { ::ffi::libusb_get_bus_number(device) };
  let address = unsafe { ::ffi::libusb_get_device_address(device) };

  println!("Bus {:03} Device {:03}", bus, address);

  return depth + 1;
}

fn get_language_ids(handle: *mut ::ffi::libusb_device_handle) -> Vec<u16> {
  let mut buf = Vec::<u8>::with_capacity(255);
  let len = unsafe { ::ffi::libusb_get_string_descriptor(handle, 0, 0, (&mut buf[..]).as_mut_ptr() as *mut c_uchar, buf.capacity() as c_int) };

  let mut languages = Vec::<u16>::new();

  if len >= 0 {
    unsafe { buf.set_len(len as usize) };

    if buf.len() >= 2 {
      let num_languages = (buf.len() - 2) / 2;
      languages.reserve(num_languages);

      let mut cursor = Cursor::new(buf);
      cursor.set_position(2);

      for _ in 0..num_languages {
        let mut bytes = Vec::<u8>::with_capacity(2);

        match cursor.read(unsafe { slice::from_raw_parts_mut((&mut bytes[..]).as_mut_ptr(), bytes.capacity()) }) {
          Ok(len) => {
            if len == 2 {
              unsafe { bytes.set_len(len) };

              let langid = (bytes[1] as u16) << 8 | (bytes[0] as u16);
              languages.push(langid)
            }
            else {
              return languages;
            }
          },
          Err(_) => return languages
        }
      }
    }
  }
  else {
    println!("libusb_get_string_descriptor: {}", len);
  }

  languages
}

fn find_readable_endpoint(device: *mut ::ffi::libusb_device,
                          transfer_type: u8,
                          vid: u16,
                          pid: u16) -> Option<Endpoint> {

  let mut descriptor_uninit: MaybeUninit<ffi::libusb_device_descriptor> = MaybeUninit::uninit();

  match unsafe { ::ffi::libusb_get_device_descriptor(device, descriptor_uninit.as_mut_ptr()) } {
    0 => {
      let device_descriptor: ffi::libusb_device_descriptor = unsafe { descriptor_uninit.assume_init() };

      println!("Check Device '{} : {}'", device_descriptor.idVendor, device_descriptor.idProduct);
      if vid <= 0 && pid <= 0 && device_descriptor.idVendor != vid && device_descriptor.idProduct != pid {
        // skip device
        println!("Device '{:03} : {:03}' skipped, because doesn't equal to '{} : {}'",
                 device_descriptor.idVendor, device_descriptor.idProduct, vid , pid);
        return None;
      }


        for i in 0..device_descriptor.bNumConfigurations {

        let configuration_config_ptr_uninit: MaybeUninit<*mut *const ffi::libusb_config_descriptor> = MaybeUninit::uninit();
        match unsafe { ::ffi::libusb_get_config_descriptor(device, i, *configuration_config_ptr_uninit.as_ptr()) } {
          0 => {
            let config_descriptor: *mut *const ffi::libusb_config_descriptor = unsafe { configuration_config_ptr_uninit.assume_init() };

            let interfaces = unsafe {
              slice::from_raw_parts((*(*config_descriptor)).interface,
                                    (*(*config_descriptor)).bNumInterfaces as usize)
            };

            for iface in interfaces {
              let settings = unsafe { slice::from_raw_parts(iface.altsetting, iface.num_altsetting as usize) };

              for iface_descriptor in settings {
                let endpoints = unsafe { slice::from_raw_parts(iface_descriptor.endpoint, iface_descriptor.bNumEndpoints as usize) };

                for endpoint_descriptor in endpoints {
                  let is_input = endpoint_descriptor.bEndpointAddress & ::ffi::LIBUSB_ENDPOINT_DIR_MASK == ::ffi::LIBUSB_ENDPOINT_IN;
                  let matches_type = endpoint_descriptor.bmAttributes & ::ffi::LIBUSB_TRANSFER_TYPE_MASK == transfer_type;

                  if is_input && matches_type {
                    return Some(Endpoint {
                      config: unsafe { (*(*config_descriptor)).bConfigurationValue },
                      iface: iface_descriptor.bInterfaceNumber,
                      setting: iface_descriptor.bAlternateSetting,
                      address: endpoint_descriptor.bEndpointAddress
                    });
                  }
                }
              }
            }
            unsafe { ffi::libusb_free_config_descriptor(*config_descriptor) };
          },
          e => println!("libusb_get_config_descriptor: {}", e)
        }
      }

      None
    },
    e => {
      println!("libusb_get_device_descriptor: {}", e);
      None
    }
  }
}

fn read_endpoint(handle: *mut ::ffi::libusb_device_handle, device: *mut ::ffi::libusb_device, endpoint: Endpoint, transfer_type: u8) {
  println!("Reading from the endpoint: {:?}", endpoint);

  let has_kernel_driver = unsafe {
    if ::ffi::libusb_kernel_driver_active(handle, endpoint.iface as c_int) == 1 {
      match ::ffi::libusb_detach_kernel_driver(handle, endpoint.iface as c_int) {
        0 => (),
        e => println!("libusb_detach_kernel_driver: {}", e)
      }

      true
    }
    else {
      false
    }
  };

  println!(" - kernel driver? {}", has_kernel_driver);

  match unsafe { ::ffi::libusb_set_configuration(handle, endpoint.config as c_int) } {
    0 => {
      println!(" - max packet size: {}", unsafe { ::ffi::libusb_get_max_packet_size(device, endpoint.address as c_uchar) });
      println!(" - max iso packet size: {}", unsafe { ::ffi::libusb_get_max_iso_packet_size(device, endpoint.address as c_uchar) });

      match unsafe { ::ffi::libusb_claim_interface(handle, endpoint.iface as c_int) } {
        0 => {
          match unsafe { ::ffi::libusb_set_interface_alt_setting(handle,
                                                                 endpoint.iface as c_int,
                                                                 endpoint.setting as c_int) } {
            0 => {
              let mut vec = Vec::<u8>::with_capacity(256);
              let timeout: c_uint = 1000;

              let mut transferred_uninit = MaybeUninit::<c_int>::zeroed();
              let transferred: c_int;

              match transfer_type {
                ::ffi::LIBUSB_TRANSFER_TYPE_INTERRUPT => {
                  match unsafe { ::ffi::libusb_interrupt_transfer(handle,
                                                                  endpoint.address as c_uchar,
                                                                  (&vec[..]).as_ptr() as *mut c_uchar,
                                                                  vec.capacity() as c_int,
                                                                  transferred_uninit.as_mut_ptr(),
                                                                  timeout) } {
                    0 => {
                      transferred = unsafe { transferred_uninit.assume_init() };
                      unsafe { vec.set_len(transferred as usize) };
                      println!(" - read: {:?}", vec);
                    },
                    e => println!("libusb_interrupt_transfer: {}", e)
                  }
                },
                ::ffi::LIBUSB_TRANSFER_TYPE_BULK => {
                  match unsafe { ::ffi::libusb_bulk_transfer(handle,
                                                             endpoint.address as c_uchar,
                                                             (&vec[..]).as_ptr() as *mut c_uchar,
                                                             vec.capacity() as c_int,
                                                             transferred_uninit.as_mut_ptr(),
                                                             timeout) } {
                    0 => {
                      transferred = unsafe { transferred_uninit.assume_init() };
                      unsafe { vec.set_len(transferred as usize) };
                      println!(" - read: {:?}", vec);
                    },
                    e => println!("libusb_interrupt_transfer: {}", e)
                  }
                },
                tt => println!(" - can't read endpoint with transfer type {}", tt)
              }
            },
            e => println!("libusb_set_interface_alt_setting: {}", e),
          }

          match unsafe { ::ffi::libusb_release_interface(handle, endpoint.iface as c_int) } {
            0 => (),
            e => println!("libusb_release_interface: {}", e)
          }
        },
        e => println!("libusb_claim_interface: {}", e)
      }
    },
    e => println!("libusb_set_configuration: {}", e)
  }



  if has_kernel_driver {
    match unsafe { ::ffi::libusb_attach_kernel_driver(handle, endpoint.iface as c_int) } {
      0 => (),
      e => println!("libusb_attach_kernel_driver: {}", e)
    }
  }
}

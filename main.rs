mod lib;

use windows_sys::{
     Win32::Foundation::*, 
    Win32::Storage::FileSystem::*, 
    Win32::System::SystemServices::*,
    Win32::Devices::Communication::DCB,
    Win32::Devices::Communication::*,
    Win32::System::IO::OVERLAPPED,
};

use core::ffi::c_void;
use std::ffi::CString;
use std::os::raw::c_char;

fn main()
{
        // Instanciate structure

    let mut my_serial_struct = lib::SerialStruct{
        serial_handle: 0,
        w_com_port: "\\\\.\\COM1\0".encode_utf16().collect(),
        baudrate: 9600,
        bytes_size: 8,
        parity: NOPARITY,
        stopbits: ONESTOPBIT,
        write_buf: CString::new("Hello, world!").unwrap(),
        read_buf: CString::new("Hello, world!").unwrap(),
    };

        // Open the Serial Port

    let ret = lib::serial_open(&mut my_serial_struct);
    match ret{
        true=>{
            println!("[*] opening serial port successfully {:?}",my_serial_struct.w_com_port);
        },
        false=>{
            return ;
         }
    }

        // Configure the Serial Port

    let ret = lib::serial_configure(&mut my_serial_struct);
    match ret{
        true=>{
            println!("[*] Configuring serial port successfully {:?}", my_serial_struct.w_com_port);
        },
        false=>{
            return ;
         }
    }

        // Close Serial Port

        lib::serial_close(my_serial_struct);

}

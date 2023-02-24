/*
Purpose: 
Implement basic Serial interface functionnalities for LabView. The library itself can help to 
not use NI VISA library or equivalent. The binding between Win32 and Labview is made with Rust.

Author: 
A.CARPENTIER
*/
use std::ffi::CString;
use core::ffi::c_void;

use user32::MessageBoxA;
use winapi::winuser::{MB_OK, MB_ICONINFORMATION};

use std::ffi::CStr;

extern crate user32;
extern crate winapi;

use windows_sys::{
    /*core::*, */ Win32::Foundation::*, 
    /*Win32::System::Threading::*, 
    */ /* Win32::UI::WindowsAndMessaging::*, 
    */ Win32::Storage::FileSystem::*, 
    Win32::System::SystemServices::*,
    Win32::Devices::Communication::DCB,
    Win32::Devices::Communication::*,
    Win32::System::IO::OVERLAPPED,
};

pub struct SerialStruct{
    serial_handle:HANDLE,
    pub w_com_port: Vec<u16>,
    baudrate:u32,
    bytes_size:u8,
    parity:u8,
    stopbits:u8,

    write_buf:CString,
    read_buf:CString,
}

pub fn serial_open(my_serial_struct: &mut SerialStruct) -> bool
{
    if my_serial_struct.serial_handle!=0
    {
        println!("Error Serial port already in use\n");
        return false;
    }
    unsafe 
    {
        my_serial_struct.serial_handle = CreateFileW(my_serial_struct.w_com_port.as_ptr(),
             GENERIC_READ | GENERIC_WRITE, 
             0, 
             std::ptr::null(), 
             OPEN_EXISTING, 
             0,
              0
            );
    }
    if my_serial_struct.serial_handle==-1
    {
        println!("Error in opening serial port");
        return false;
    }
    else
    {
        return true;
    }
}

pub fn serial_configure(my_serial_struct: &mut SerialStruct) ->  bool
{
    let mut res:BOOL = 0;

    let mut dcb = DCB{ 
        DCBlength: 0,
        BaudRate: 0,
        _bitfield: 0,
        wReserved: 0, 
        XonLim: 0, 
        XoffLim: 0, 
        ByteSize: 0, 
        Parity: 0, 
        StopBits: 0, 
        XonChar: 0, 
        XoffChar: 0, 
        ErrorChar: 0, 
        EofChar: 0, 
        EvtChar: 0, 
        wReserved1: 0 
       };

       let raw = &mut dcb as *mut DCB;
    
    unsafe 
    {
        res = GetCommState(my_serial_struct.serial_handle, raw);

        (*raw).BaudRate = my_serial_struct.baudrate;
        (*raw).ByteSize = my_serial_struct.bytes_size;
        (*raw).Parity = my_serial_struct.parity;
        (*raw).StopBits = my_serial_struct.stopbits;
    }

    if res == 0
    {
        println!("Error at GetCommState() on serial port");
        return false;
    }
    else
    {
        unsafe{
            res = SetCommState(my_serial_struct.serial_handle, raw);
        }
        if res == 0
        {
            println!("Error at SetCommState() on serial port");
            return false;
        }
        else
        {
            return true;
        }
    }
}

pub fn serial_write(my_serial_struct: &mut SerialStruct) -> bool
{
    let mut res = 0;
    
    unsafe{
        let mut byteswritten:*mut u32 =std::ptr::null_mut();
        let mut ov:*mut OVERLAPPED =std::ptr::null_mut();

        // String
        //let mut raw = c_string.into_raw();

        //let mut_ref= &mut raw;

        //let raw_ptr= mut_ref as *mut _;

        //let void_cast: *mut c_void = raw_ptr as *mut c_void;    
        //*mut c_void

        let lp_buf = &my_serial_struct.write_buf;

        res =WriteFile(my_serial_struct.serial_handle,        
            lp_buf.as_ptr() as *mut c_void,     
            6, 
            byteswritten, 
            ov
        );
    }
    if res == 0
    {
        println!("Error at GetCommState() on serial port");
        return false;
    }
    else
    {
        return true;
    }
}

pub fn serial_close(my_serial_struct :SerialStruct)->bool
{
    unsafe
    {
        CloseHandle(my_serial_struct.serial_handle);
    }
    println!("[*] Serial port Closed");
    return true;
}

#[no_mangle]
pub extern "C" fn comm_test(addr: *const char)
{
    unsafe{
        let lp_text =CStr::from_ptr(addr as *const i8);

        let lp_caption = CString::new("MessageBox Example").unwrap();
    

            MessageBoxA(
                std::ptr::null_mut(),
                lp_text.as_ptr(),
                lp_caption.as_ptr(),
                MB_OK | MB_ICONINFORMATION);

    }
}

#[no_mangle]
pub extern "C" fn comm_idn(c_addr: *const char)
{
    let mut addr ;

    unsafe{
        let addr =CStr::from_ptr(c_addr as *const i8);
    
        let len = addr.to_bytes().len();

        let addr_string = String::from_utf8_lossy(addr.to_bytes()).to_string();

        addr = addr_string.encode_utf16().collect();
    }

    let mut my_serial_struct = SerialStruct{
        serial_handle: 0,
        w_com_port: addr,
        baudrate: 9600,
        bytes_size: 8,
        parity: NOPARITY,
        stopbits: ONESTOPBIT,
        write_buf: CString::new("*IDN?\r\n").unwrap(),
        read_buf: CString::new("").unwrap(),
    };
            // Open the Serial Port
    
        let ret = serial_open(&mut my_serial_struct);
        match ret{
            true=>{
                println!("[*] opening serial port successfully {:?}",my_serial_struct.w_com_port);
            },
            false=>{
                return ;
             }
        }
    
            // Configure the Serial Port
    
        let ret = serial_configure(&mut my_serial_struct);
        match ret{
            true=>{
                println!("[*] Configuring serial port successfully {:?}", my_serial_struct.w_com_port);
            },
            false=>{
                serial_close(my_serial_struct);
                return ;
             }
        }

            // Send *IDN?\r\n to the instrument

        let ret = serial_write(&mut my_serial_struct);
        match ret{
            true=>{
                println!("[*] Sending *IDN? on serial port successfully {:?}", my_serial_struct.w_com_port);
            },
            false=>{
                serial_close(my_serial_struct);
                return ;
             }
        }
    
            // Close Serial Port
    
            serial_close(my_serial_struct);
   
       
}

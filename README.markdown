# rust-jsonnet
====

rust-jsonnet - The Google Jsonnet( operation data template language) for rust

Google jsonnet documet: (http://google.github.io/jsonnet/doc/)


Parse the file

```
extern crate libc;
extern crate jsonnet;
use libc::c_char;
use std::ffi::CStr;
use libc::size_t;
use std::str;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use jsonnet::ffi::command::{ Jsonnet };

pub fn ctos(msg_buf : *const c_char)-> String{
let msg_str: &CStr = unsafe { CStr::from_ptr(msg_buf) };
let buf: &[u8] = msg_str.to_bytes();
let str_buf: &str = str::from_utf8(buf).unwrap();
let msg_data: String = str_buf.to_owned();
return msg_data;
}

pub fn evaluate_file(){
let mut error : size_t;
error = 0;
let msg_buf: *const c_char = Jsonnet::evaluate_file("./t2.jsonnet".as_ptr() as *const c_char,&mut error);
let msg_data: String = ctos(msg_buf);
println!("{:?}", msg_data);
}

fn main() {
evaluate_file();
Jsonnet::destroy();
}
```

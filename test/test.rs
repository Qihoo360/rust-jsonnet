extern crate libc; 
extern crate jsonnet;
use libc::c_char;
use jsonnet::ffi::command::{ Jsonnet }; 

#[test]
pub fn follows_redirects() {
	let msg  : *const c_char = Jsonnet::version();
	println!("{:?}",msg ); 
}

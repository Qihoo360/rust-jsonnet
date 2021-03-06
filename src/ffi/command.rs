#[warn(unused_imports)]
extern crate libc; 
use libc::size_t; 
use libc::c_char;
use std::ffi::CStr;
#[warn(unused_mut)]
#[repr(C)]
struct JsonnetVm;
static mut _VM_LOCK:bool = false;
static mut _VM :*mut JsonnetVm  = 0 as *mut JsonnetVm; 
pub type JsonnetResult = Result<String, String>;
#[link(name = "jsonnet")]
extern {
	#[warn(dead_code)] 
	fn jsonnet_version()	 ->  *const c_char;
	fn jsonnet_make()	 -> *mut JsonnetVm;
	fn jsonnet_evaluate_file(vm: *mut JsonnetVm,filename: *const c_char,error:*mut size_t) -> *const c_char; 
	fn jsonnet_evaluate_snippet(vm: *mut JsonnetVm,filename: *const c_char,snippet: *const c_char,error:*mut size_t) ->*const c_char; 
	fn jsonnet_destroy(vm: *mut JsonnetVm);
} 
#[warn(private_no_mangle_fns)]
fn ctos(msg : *const c_char)-> String{
	let buf = unsafe { CStr::from_ptr(msg).to_bytes() };
	let val = String::from_utf8(buf.to_vec()).unwrap();
	return val;
}
pub struct Jsonnet;
impl Jsonnet { 

	#[no_mangle]
	pub fn evaluate_file(filename:  *const c_char) -> JsonnetResult {
		unsafe { 
			if !_VM_LOCK {
				_VM = jsonnet_make();
				_VM_LOCK = true;
			}
			let mut error : size_t = 0 as size_t;  
			let data = jsonnet_evaluate_file(_VM,filename,&mut error); 
			if error == 0 {
				Ok(ctos(data))
			}else{
				Err(ctos(data))
			}
		}
	}

	#[no_mangle]
	pub fn evaluate_snippet(snippet: *const c_char) -> JsonnetResult {
		unsafe { 
			if !_VM_LOCK {
				_VM = jsonnet_make();
				_VM_LOCK = true;
			} 
			let mut error : size_t = 0 as size_t;
			let data = jsonnet_evaluate_snippet(_VM,"snippet".as_ptr() as *const c_char,snippet,&mut error);  
			if error == 0 {
				Ok(ctos(data))
			}else{
				Err(ctos(data))
			}
			//return data;
		}
	}
	#[no_mangle]
	#[warn(dead_code)]
	pub fn version() -> *const c_char {
		unsafe { 
			return jsonnet_version();
		}
	}
	#[no_mangle]
	#[warn(dead_code)]
	pub fn destroy()  {
		unsafe { 
			jsonnet_destroy(_VM);
			_VM_LOCK = false;
		}
	}

}
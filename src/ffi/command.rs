#[warn(unused_imports)]
extern crate libc; 
use libc::size_t; 
use libc::c_char;
use libc::c_void;
#[warn(unused_mut)]
#[repr(C)]
struct JsonnetVm;
static mut _VM_LOCK:bool = false;
static mut _VM :*mut JsonnetVm  = 0 as *mut JsonnetVm;
#[link(name = "jsonnet")]
extern {
	#[warn(dead_code)] 
	fn jsonnet_version()	 ->  *const c_char;
	fn jsonnet_make()	 -> *mut JsonnetVm;
	fn jsonnet_evaluate_file(vm: *mut JsonnetVm,filename: *const c_char,error:*mut size_t) -> *const c_char; 
	fn jsonnet_evaluate_snippet(vm: *mut JsonnetVm,filename: *const c_char,snippet: *const c_char,error:*mut size_t) -> *const c_char; 
	fn jsonnet_destroy(vm: *mut JsonnetVm);
	fn jsonnet_realloc(vm: *mut JsonnetVm,buf:*const c_char,sz:size_t) ->  *const c_char;
} 

pub struct Jsonnet;
impl Jsonnet { 
	#[no_mangle]
	pub fn evaluate_file(filename:  *const c_char,error: *mut size_t) -> *const c_char {
		unsafe { 
			if !_VM_LOCK {
				_VM = jsonnet_make();
				_VM_LOCK = true;
			}
			let sz:size_t=0;
			let data = jsonnet_evaluate_file(_VM,filename, error);
			let oc = jsonnet_realloc(_VM,data,sz);
			return data;
		}
	}

	#[no_mangle]
	pub fn evaluate_snippet(snippet: *const c_char,error:*mut size_t) -> *const c_char {
		unsafe { 
			if !_VM_LOCK {
				_VM = jsonnet_make();
				_VM_LOCK = true;
			}
			let sz:size_t=0;
			let data = jsonnet_evaluate_snippet(_VM,"snippet".as_ptr() as *const c_char,snippet, error);
			let oc = jsonnet_realloc(_VM,data,sz);
			return data;
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
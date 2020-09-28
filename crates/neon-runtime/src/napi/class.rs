use std::os::raw::c_void;
use call::CCallback;
use raw::{Env, Local};

pub unsafe fn get_class_map(_isolate: Env) -> *mut c_void { unimplemented!() }

pub unsafe fn set_class_map(_isolate: Env, _map: *mut c_void, _free_map: *mut c_void) { unimplemented!() }

pub unsafe fn create_base(_isolate: Env,
                                     _allocate: CCallback,
                                     _construct: CCallback,
                                     _call: CCallback,
                                     _drop: extern "C" fn(*mut c_void)) -> *mut c_void { unimplemented!() }

pub unsafe fn get_name(_base_out: &mut *mut u8, _isolate: Env, _metadata: *const c_void) -> usize { unimplemented!() }

pub unsafe fn set_name(_isolate: Env, _metadata: *mut c_void, _name: *const u8, _byte_length: u32) -> bool { unimplemented!() }

pub unsafe fn throw_call_error(_isolate: Env, _metadata: *mut c_void) { unimplemented!() }

pub unsafe fn throw_this_error(_isolate: Env, _metadata: *mut c_void) { unimplemented!() }

pub unsafe fn add_method(_isolate: Env, _metadata: *mut c_void, _name: *const u8, _byte_length: u32, _method: Local) -> bool { unimplemented!() }

pub unsafe fn metadata_to_constructor(_out: &mut Local, _isolate: Env, _metadata: *mut c_void) -> bool { unimplemented!() }

// FIXME: get rid of all the "kernel" nomenclature

pub unsafe fn get_allocate_kernel(_data: *mut c_void) -> *mut c_void { unimplemented!() }

pub unsafe fn get_construct_kernel(_data: *mut c_void) -> *mut c_void { unimplemented!() }

pub unsafe fn get_call_kernel(_data: *mut c_void) -> *mut c_void { unimplemented!() }

pub unsafe fn constructor(_out: &mut Local, _ft: Local) -> bool { unimplemented!() }

pub unsafe fn has_instance(_metadata: *mut c_void, _v: Local) -> bool { unimplemented!() }

pub unsafe fn get_instance_internals(_obj: Local) -> *mut c_void { unimplemented!() }

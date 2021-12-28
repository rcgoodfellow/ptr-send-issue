use std::thread::spawn;
use std::os::raw::{c_void, c_int, c_char};
use std::ffi::CString;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct DlHandle(pub *mut c_void);
unsafe impl Send for DlHandle {}

extern "C" {
    fn dlopen(pathname: *const c_char, mode: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, name: *const c_char) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> c_int;
}

fn main() {

    // open a handle to a set of global set of symbol objects for this program.
    let h = unsafe{ DlHandle(dlopen(ptr::null(), 2)) };
    do_work(h);

}

fn do_work(h: DlHandle) {

    // find the read and blueberry functions in the first thread
    let h0 = h.clone();
    let j0 = spawn(move || {
        let name = CString::new("read").unwrap();
        let fp = unsafe{ dlsym(h0.0, name.as_c_str().as_ptr()) };
        println!("read: {:?}", fp);

        let name = CString::new("blueberry").unwrap();
        let fp = unsafe{ dlsym(h0.0, name.as_c_str().as_ptr()) };
        println!("blueberry: {:?}", fp);
    });

    // find the write and muffin functions in the second thread
    let h1 = h.clone();
    let j1 = spawn(move || {
        let name = CString::new("write").unwrap();
        let fp = unsafe{ dlsym(h1.0, name.as_c_str().as_ptr()) };
        println!("write: {:?}", fp);

        let name = CString::new("muffin").unwrap();
        let fp = unsafe{ dlsym(h1.0, name.as_c_str().as_ptr()) };
        println!("muffin: {:?}", fp);
    });

    // wait for completion
    j0.join().unwrap();
    j1.join().unwrap();

    unsafe{ dlclose(h.0) };

}

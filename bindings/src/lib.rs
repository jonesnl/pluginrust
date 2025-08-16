#![allow(unused)]

mod bindings;

pub use bindings::*;
use windows::Win32::System::LibraryLoader::{
    LoadLibraryW,
    GetProcAddress,
};
use windows_strings::*;

fn blah() {
    unsafe {
        let _mod_handle = LoadLibraryW(w!("webauthn.dll")).expect("Loading webauthn.dll failed");
        //let addr = GetProcAddress(mod_handle, w!("")).expect("Couldn't find func");
    }
}
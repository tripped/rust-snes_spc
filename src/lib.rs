extern crate libc;

use libc::{c_char, c_short, c_int};

pub enum SpcHandle {}

#[link(name = "snes_spc")]
extern {
    // Instantiate a new SPC emulator and return a handle to it.
    fn spc_new() -> *mut SpcHandle;

    // Free an SPC emulator given its handle.
    fn spc_delete(spc: *mut SpcHandle);

    // Load SPC data into emulator.
    fn spc_load_spc(spc: *mut SpcHandle, data: *const u8, len: c_int)
        -> *const c_char;

    // Clear SPC echo buffer.
    fn spc_clear_echo(spc: *mut SpcHandle);

    // Emulate SPC output, recording `count` 16-bit samples to `out`
    fn spc_play(spc: *mut SpcHandle, count: c_int, out: *mut c_short)
        -> *const c_char;
}

/// A simple, safe wrapper around an opaque SNES_SPC handle.
pub struct SnesSpc {
    handle: *mut SpcHandle
}

impl SnesSpc {
    pub fn new() -> SnesSpc {
        unsafe {
            SnesSpc {
                handle: spc_new()
            }
        }
    }

    pub fn load_spc(&mut self, data: &[u8]) {
        unsafe {
            spc_load_spc(self.handle, data.as_ptr(), data.len() as c_int);
        }
    }

    pub fn clear_echo(&mut self) {
        unsafe {
            spc_clear_echo(self.handle);
        }
    }

    pub fn play(&mut self, count: u32, out: &mut [i16]) {
        unsafe {
            // TODO: check that out is big enough :)
            spc_play(self.handle, count as c_int, out.as_mut_ptr());
        }
    }
}

#[test]
fn can_call_unsafe_api_without_exploding() {
    unsafe {
        let mut spc = spc_new();
        spc_delete(spc);
    }
}

#[test]
fn can_use_safe_wrapper_without_being_stabbed() {
    let mut spc = SnesSpc::new();
    spc.clear_echo();
    // Drop it!
}

extern crate libc;

use libc::{c_char, c_short, c_int};

pub enum SpcHandle {}

#[link(name = "snes_spc")]
extern {
    // Instantiate a new SPC emulator and return a handle to it.
    pub fn spc_new() -> *mut SpcHandle;

    // Free an SPC emulator given its handle.
    pub fn spc_delete(spc: *mut SpcHandle);

    // Load SPC data into emulator.
    pub fn spc_load_spc(spc: *mut SpcHandle, data: *const u8, len: c_int)
        -> *const c_char;

    // Clear SPC echo buffer.
    pub fn spc_clear_echo(spc: *mut SpcHandle);

    // Emulate SPC output, recording `count` 16-bit samples to `out`
    pub fn spc_play(spc: *mut SpcHandle, count: c_int, out: *mut c_short)
        -> *const c_char;
}

#[test]
fn it_works() {
    unsafe {
        let mut spc = spc_new();
        spc_delete(spc);
    }
}

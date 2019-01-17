extern crate libc;

use std::convert::From;
use std::error;
use std::ffi::CStr;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::result;

use libc::{c_char, c_short, c_int};

/// The error type for SPC operations. Most of the wrapped C `snes_spc`
/// functions have the possibility of returning an error string from the
/// underlying library; in those cases the returned error is just an
/// uninterpreted copy of that string. Other members of the enumeration
/// represent errors arising from the wrapper functions themselves.
#[derive(Debug)]
pub enum SpcError {
    Internal(String),
    Io(io::Error),
    OddSizeBuffer,
}

impl fmt::Display for SpcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SpcError::Io(ref e) => write!(f, "IO error: {}", e),
            SpcError::Internal(ref e) => write!(f, "Internal SPC error: {}", e),
            SpcError::OddSizeBuffer => write!(f, "SPC error: {}", self)
        }
    }
}

impl error::Error for SpcError {
    fn description(&self) -> &str {
        match *self {
            SpcError::Internal(ref e) => &e,
            SpcError::Io(ref e) => e.description(),
            SpcError::OddSizeBuffer => "sample buffer size not multiple of two",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SpcError::Internal(_) => None,
            SpcError::Io(ref e) => Some(e),
            SpcError::OddSizeBuffer => None,
        }
    }
}

impl From<io::Error> for SpcError {
    fn from(err: io::Error) -> SpcError {
        SpcError::Io(err)
    }
}

impl From<String> for SpcError {
    fn from(err: String) -> SpcError {
        SpcError::Internal(err)
    }
}

/// Convert an ffi C-string into our error type. Deal with the rigmarole
/// of cloning into an owned string.
impl From<*const c_char> for SpcError {
    fn from(err: *const c_char) -> SpcError {
        unsafe {
            SpcError::Internal(String::from_utf8_lossy(
                CStr::from_ptr(err).to_bytes()).into_owned())
        }
    }
}

/// The result type for SPC operations that may fail.
pub type Result<T> = result::Result<T, SpcError>;

/// Convert a possibly null ffi C-string into our Result<()> type.
/// N.B.: this is not an implementation of `std::convert::From` because
/// Rust's coherency rules prohibit us from defining external traits for
/// `snes_spc::Result`, which is just an alias of `std::result::Result`.
/// This makes tiny kittens sad, but they try their best to understand.
fn result_from(err: *const c_char) -> Result<()> {
    if err.is_null() {
        Ok(())
    } else {
        Err(SpcError::from(err))
    }
}

/// A type for the internal library's opaque struct SNES_SPC
enum SpcHandle {}

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

/// SnesSpc's internal handle is never exposed except through its safe
/// implementation, so it should be save to move to another thread.
unsafe impl Send for SnesSpc {}

impl SnesSpc {
    pub fn new() -> SnesSpc {
        unsafe {
            SnesSpc {
                handle: spc_new()
            }
        }
    }

    /// Creates a new `SnesSpc` using a byte slice as its initial SPC data.
    pub fn from_data(data: &[u8]) -> Result<SnesSpc> {
        let mut spc = SnesSpc::new();
        spc.load_spc(data)?;
        Ok(spc)
    }

    /// Creates a new SPC emulator initially loaded with the contents of a
    /// file at the given path.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<SnesSpc> {
        let mut data = Vec::new();
        let mut file = File::open(path)?;
        file.read_to_end(&mut data)?;
        SnesSpc::from_data(&data)
    }

    /// Loads data into the SPC emulator.
    pub fn load_spc(&mut self, data: &[u8]) -> Result<()> {
        unsafe {
            result_from(spc_load_spc(
                self.handle, data.as_ptr(), data.len() as c_int))
        }
    }

    /// Clears the SPC emulator's echo buffer. This can be useful if loading
    /// SPC files that have garbage in the initial echo buffer.
    pub fn clear_echo(&mut self) {
        unsafe {
            spc_clear_echo(self.handle);
        }
    }

    /// Emulate samples into the specified buffer. The number of samples
    /// emulated will be equal to the size of the buffer, which must be a
    /// multiple of two (to accommodate stereo channels).
    pub fn play(&mut self, out: &mut [i16]) -> Result<()> {
        unsafe {
            let len = out.len() as c_int;
            if len % 2 != 0 {
                Err(SpcError::OddSizeBuffer)
            } else {
                result_from(spc_play(self.handle, len, out.as_mut_ptr()))
            }
        }
    }
}

impl Drop for SnesSpc {
    fn drop(&mut self) {
        unsafe {
            spc_delete(self.handle);
        }
    }
}

#[test]
fn can_call_unsafe_api_without_exploding() {
    unsafe {
        let spc = spc_new();
        spc_delete(spc);
    }
}

#[test]
fn can_use_safe_wrapper_without_being_stabbed() {
    let mut spc = SnesSpc::new();
    spc.clear_echo();
    // Drop it!
}

#[test]
fn from_file_with_bad_path_produces_io_error() {
    match SnesSpc::from_file("") {
        Err(SpcError::Io(_)) => {},
        _ => { panic!("Expected io::Error!") }
    }
}

#[test]
fn odd_size_buffer_produces_playback_error() {
    let mut spc = SnesSpc::new();
    let mut buf = vec![0;101];
    match spc.play(&mut buf) {
        Err(SpcError::OddSizeBuffer) => {},
        _ => { panic!("Expected OddSizeBuffer error!") }
    }
}

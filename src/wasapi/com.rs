//! Handles COM initialization and cleanup.

use super::check_result;
use std::ptr;

use super::winapi::um::objbase::{COINIT_MULTITHREADED};
use super::winapi::um::combaseapi::{CoInitializeEx, CoUninitialize};

thread_local!(static COM_INITIALIZED: ComInitialized = {
    unsafe {
        // this call can fail if another library initialized COM in single-threaded mode
        let result = check_result(CoInitializeEx(ptr::null_mut(), COINIT_MULTITHREADED));
        ComInitialized {
            initialized: result.is_ok(),
            _marker: ptr::null_mut(),
        }
    }
});

/// RAII object that guards the fact that COM is initialized.
///
// We store a raw pointer because it's the only way at the moment to remove `Send`/`Sync` from the
// object.
struct ComInitialized {
    initialized: bool,
    _marker: *mut (),
}

impl Drop for ComInitialized {
    #[inline]
    fn drop(&mut self) {
        if self.initialized {
            unsafe { CoUninitialize() };
        }
    }
}

/// Ensures that COM is initialized in this thread.
#[inline]
pub fn com_initialized() {
    COM_INITIALIZED.with(|_| {});
}

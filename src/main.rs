#[link(name = "ntdll")]
extern "system" {
    pub fn NtRaiseHardError(
        ErrorStatus: u32,
        NumberOfParameters: u32,
        UnicodeStringParameterMask: *mut u32,
        Parameters: *mut u32,
        ResponseOption: u32,
        Response: *mut u32,
    ) -> u32;
}

use std::ptr;

fn main() {
    unsafe {
        const STATUS_SYSTEM_PROCESS_TERMINATED: u32 = 0xC000021A;
        let mut response: u32 = 0;

        let result = NtRaiseHardError(
            STATUS_SYSTEM_PROCESS_TERMINATED, // Error code
            0,                                // Number of parameters
            ptr::null_mut(),                  // Unicode string mask
            ptr::null_mut(),                  // Parameters array
            0,                                // Response option (0 = show error)
            &mut response as *mut u32,        // Response
        );

        if result == 0 {
            println!("System is crashing...");
        } else {
            println!("Failed to trigger system crash.");
        }
    }
}

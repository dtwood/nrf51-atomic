#![no_std]

use core::ptr;

struct DisableInterrupts();

impl DisableInterrupts {
    pub fn new() -> DisableInterrupts {
        // TODO: disable interrupts
        DisableInterrupts()
    }
}

impl Drop for DisableInterrupts {
    fn drop(&mut self) {
        // TODO: re-enable interrupts
    }
}

fn disable_interrupts() -> DisableInterrupts {
    DisableInterrupts::new()
}

/// These builtins perform an atomic compare and swap. That is, if the current value of *ptr is
/// oldval, then write newval into *ptr.
///
/// The “bool” version returns true if the comparison is successful and newval was written. The
/// “val” version returns the contents of *ptr before the operation.
#[no_mangle]
pub unsafe extern "C" fn __sync_val_compare_and_swap_1(ptr_: *mut u8, oldval: u8, newval: u8) -> u8 {
    let _interrupts_disabled = disable_interrupts();
    let startval = ptr::read(ptr_);

    if startval == oldval {
        ptr::write(ptr_, newval);
    }

    startval
}

#[no_mangle]
pub unsafe extern "C" fn __sync_val_compare_and_swap_4(ptr_: *mut u32, oldval: u32, newval: u32) -> u32 {
    let _interrupts_disabled = disable_interrupts();
    let startval = ptr::read(ptr_);

    if startval == oldval {
        ptr::write(ptr_, newval);
    }

    startval
}

/// This builtin, as described by Intel, is not a traditional test-and-set operation, but rather an
/// atomic exchange operation. It writes value into *ptr, and returns the previous contents of
/// *ptr.
///
/// Many targets have only minimal support for such locks, and do not support a full exchange
/// operation. In this case, a target may support reduced functionality here by which the only
/// valid value to store is the immediate constant 1. The exact value actually stored in *ptr is
/// implementation defined.
///
/// This builtin is not a full barrier, but rather an acquire barrier. This means that references
/// after the builtin cannot move to (or be speculated to) before the builtin, but previous memory
/// stores may not be globally visible yet, and previous memory loads may not yet be satisfied.

#[no_mangle]
pub unsafe extern "C" fn __sync_lock_test_and_set_1(ptr_: *mut u8, value: u8) -> u8 {
    let _interrupts_disabled = disable_interrupts();

    ptr::replace(ptr_, value)
}

#[no_mangle]
pub unsafe extern "C" fn __sync_lock_test_and_set_4(ptr_: *mut u32, value: u32) -> u32 {
    let _interrupts_disabled = disable_interrupts();

    ptr::replace(ptr_, value)
}

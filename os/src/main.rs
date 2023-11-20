//! The main module and entrypoint
//!
//! The operating system and app also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality [`clear_bss()`]. (See its source code for
//! details.)
//!
//! We then call [`println!`] to display `Hello, world!`.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;
use log::*;

#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;

global_asm!(include_str!("entry.asm"));

/// clear BSS segment
pub fn clear_bss() {
    extern "C" {
        static mut sbss: u64;
        static mut ebss: u64;
    }
    unsafe {
        let mut ptr = &mut sbss as *mut u64;
        let end = &mut ebss as *mut u64;
        while ptr < end {
            ptr.write_volatile(0);
            ptr = ptr.offset(1);
        }
    }
}

/// the rust entry-point of os
#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        static mut stext: u64; // begin addr of text segment
        static mut etext: u64; // end addr of text segment
        static mut srodata: u64; // start addr of Read-Only data segment
        static mut erodata: u64; // end addr of Read-Only data ssegment
        static mut sdata: u64; // start addr of data segment
        static mut edata: u64; // end addr of data segment
        static mut sbss: u64; // start addr of BSS segment
        static mut ebss: u64; // end addr of BSS segment
        static mut boot_stack_lower_bound: u64; // stack lower bound
        static mut boot_stack_top: u64; // stack top
    }
    clear_bss();
    logging::init();
    println!("[kernel] Hello, world!");
    trace!(
        "[kernel] .text [{:#x}, {:#x})",
        unsafe { stext as usize },
        unsafe { etext as usize }
    );
    debug!(
        "[kernel] .rodata [{:#x}, {:#x})",
        unsafe { srodata as usize },
        unsafe { erodata as usize }
    );
    info!(
        "[kernel] .data [{:#x}, {:#x})",
        unsafe { sdata as usize },
        unsafe { edata as usize }
    );
    warn!(
        "[kernel] boot_stack top=bottom={:#x}, lower_bound={:#x}",
        unsafe { boot_stack_top as usize },
        unsafe { boot_stack_lower_bound as usize }
    );
    error!(
        "[kernel] .bss [{:#x}, {:#x})",
        unsafe { sbss as usize },
        unsafe { ebss as usize }
    );

    // CI autotest success: sbi::shutdown(false)
    // CI autotest failed : sbi::shutdown(true)
    sbi::shutdown(false)
}

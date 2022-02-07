#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static BOOTMESSAGE: &[u8] = b"TROLLED!";


// this function is the entry point, since the linker looks for a function
// named `_start` by default
#[no_mangle] 
pub extern "C" fn _start() -> ! {
    //Cast the integer 0xb8000 into a raw pointer
    let vga_buffer = 0xb8000 as *mut u8;

    //use the offset method to write the string byte and the corrosponding color byte (0xb is cyan)
    for (i, &byte) in BOOTMESSAGE.iter().enumerate(){
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    
    loop {}
}
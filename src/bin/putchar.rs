#![feature(asm_const)]
#![no_std]
#![no_main]

const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_TERMNIATE: usize = 3;
static mut ABI_TABLE: u32 = 0;

unsafe fn hello() {
    core::arch::asm!("
        li      t0, {abi_num}
        slli    t0, t0, 3
        add     t1, a7, t0
        ld      t1, (t1)
        jalr    t1",
        // clobber_abi("C"),
        abi_num = const SYS_HELLO,
    )
}

unsafe fn put_char(c: char) {
    let arg0: u8 = c as u8;
    core::arch::asm!("
        li      t0, {abi_num}
        slli    t0, t0, 3
        add     t1, a7, t0
        ld      t1, (t1)
        jalr    t1",
        // clobber_abi("C"),
        abi_num = const SYS_PUTCHAR,
        in("a0") arg0,
    )
}

fn puts(s: &str) {
    for c in s.as_bytes() {
        unsafe {put_char(*c as char);}
    }    
}

unsafe fn terminate() {
   core::arch::asm!("
        li      t0, {abi_num}
        slli    t0, t0, 3
        add     t1, a7, t0
        ld      t1, (t1)
        jalr    t1",
        // clobber_abi("C"),
        abi_num = const SYS_TERMNIATE,
    ) 
}

#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _start() {
    hello();
    put_char('A');
    puts("exercise 5");
    terminate();
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

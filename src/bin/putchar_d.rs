#![feature(asm_const)]
#![no_std]
#![no_main]

const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
// const SYS_TERMNIATE: usize = 3;

#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _start() {
    let arg0: u8 = b'D';
    core::arch::asm!("
        li      t0, {abi_num}
        slli    t0, t0, 3
        add     t1, a7, t0
        ld      t1, (t1)
        jalr    t1",
        clobber_abi("C"),
        abi_num = const SYS_PUTCHAR,
        in("a0") arg0,
    )
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

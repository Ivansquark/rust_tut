#![no_std]
#![no_main]

mod startup;
//mod uart;

use core::ptr;

const RCC_AHB1: u32 = 0x4002_3830;
const RCC_AHB1_GPIOD: u32 = 1 << 3;
const GPIOD_BASE: u32 = 0x4002_0C00;
const GPIOD_ODR: u32 = 0x4002_0C14;
const GPIOD_MODER_OUT_12: u32 = 1 << 24;
const GPIOD_12: u32 = 1 << 12;

static mut X: u32 = 0;
#[unsafe(no_mangle)]
fn main() -> ! {
    unsafe {
        // GPIOD12 init
        ptr::write_volatile((RCC_AHB1) as *mut u32, RCC_AHB1_GPIOD);

        let moder = (GPIOD_BASE) as *mut u32;
        ptr::write_volatile(moder, ptr::read_volatile(moder) | (GPIOD_MODER_OUT_12));
    }

    loop {
        unsafe {
            X = X + 1;
            if X % 50 == 0 {
                delay(2000_000);
            }
            let odr = ptr::read_volatile(GPIOD_ODR as *mut u32);
            ptr::write_volatile((GPIOD_ODR) as *mut u32, odr | GPIOD_12);
            delay(100_000);

            let reset = ptr::read_volatile(GPIOD_ODR as *mut u32);
            ptr::write_volatile((GPIOD_ODR) as *mut u32, odr & !reset);
            delay(100_000);
        }
    }
}

//fn delay(count: u32) {
//    for _ in 0..count {
//    }
//}

//fn delay(count: u32) {
//    let mut i = count;
//    while i > 0 {
//        i -= 1;
//        //core::hint::black_box(i);
//    }
//}

fn delay(count: u32) {
    unsafe {
        core::arch::asm!(
            "1:",
            "subs {0}, {0}, #1",
            "bne 1b",
            inout(reg) count => _,
            options(nostack)
        );
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn USART1_Handler() {
    //
}

use core::panic::PanicInfo;
use core::ptr;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

unsafe extern "C" {
    static mut _sbss: u32;
    static mut _ebss: u32;
    static mut _sdata: u32;
    static mut _edata: u32;
    static _sidata: u32;
    static _estack: u32;
}

#[allow(dead_code)]
type ExceptionHandler = unsafe extern "C" fn() -> !;
#[unsafe(link_section = ".isr_vector")]
#[used] // optimizer delete it without used
#[allow(dead_code)]
pub static ISR_VECTOR: [ExceptionHandler; 97] = [
    reset_handler,
    nmi_handler,
    hardfault_handler,
    memorymanagement_handler,
    busfault_handler,
    usagefault_handler,
    default_handler, // Reserved
    default_handler, // Reserved
    default_handler, // Reserved
    default_handler, // Reserved
    svcall_handler,
    debugmonitor_handler,
    default_handler, // Reserved
    pendsv_handler,
    systick_handler,
    windowwatchdog_irq_handler,
    pvd_irq_handler,
    tamper_irq_handler,
    rtc_irq_handler,
    flash_irq_handler,
    rcc_irq_handler,
    exti0_irq_handler,
    exti1_irq_handler,
    exti2_irq_handler,
    exti3_irq_handler,
    exti4_irq_handler,
    dma1_channel1_irq_handler,
    dma1_channel2_irq_handler,
    dma1_channel3_irq_handler,
    dma1_channel4_irq_handler,
    dma1_channel5_irq_handler,
    dma1_channel6_irq_handler,
    dma1_channel7_irq_handler,
    adc1_2_irq_handler,
    usb_hp_can_tx_irq_handler,
    usb_lp_can_rx0_irq_handler,
    can_rx1_irq_handler,
    can_sce_irq_handler,
    exti9_5_irq_handler,
    tim1_brk_irq_handler,
    tim1_up_irq_handler,
    tim1_trg_com_irq_handler,
    tim1_cc_irq_handler,
    tim2_irq_handler,
    tim3_irq_handler,
    tim4_irq_handler,
    i2c1_ev_irq_handler,
    i2c1_er_irq_handler,
    i2c2_ev_irq_handler,
    i2c2_er_irq_handler,
    spi1_irq_handler,
    spi2_irq_handler,
    usart1_irq_handler,
    usart2_irq_handler,
    usart3_irq_handler,
    exti15_10_irq_handler,
    rtc_alarm_irq_handler,
    otg_fs_wkup_irq_handler,
    tim8_brk_tim12_irq_handler,
    tim8_up_tim13_irq_handler,
    tim8_trg_com_tim14_irq_handler,
    tim8_cc_irq_handler,
    dma1_stream7_irq_handler,
    fsmc_irq_handler,
    sdio_irq_handler,
    tim5_irq_handler,
    spi3_irq_handler,
    uart4_irq_handler,
    uart5_irq_handler,
    tim6_dac_irq_handler,
    tim7_irq_handler,
    dma2_stream0_irq_handler,
    dma2_stream1_irq_handler,
    dma2_stream2_irq_handler,
    dma2_stream3_irq_handler,
    dma2_stream4_irq_handler,
    eth_irq_handler,
    eth_wkup_irq_handler,
    can2_tx_irq_handler,
    can2_rx0_irq_handler,
    can2_rx1_irq_handler,
    can2_sce_irq_handler,
    otg_fs_irq_handler,
    dma2_stream5_irq_handler,
    dma2_stream6_irq_handler,
    dma2_stream7_irq_handler,
    usart6_irq_handler,
    i2c3_ev_irq_handler,
    i2c3_er_irq_handler,
    otg_hs_ep1_out_irq_handler,
    otg_hs_ep1_in_irq_handler,
    otg_hs_wkup_irq_handler,
    otg_hs_irq_handler,
    dcmi_irq_handler,
    default_irq_handler,
    hash_rng_irq_handler,
    fpu_irq_handler,
];

#[allow(dead_code)]
unsafe extern "C" {
    fn nmi_handler() -> !;
    fn memorymanagement_handler() -> !;
    fn busfault_handler() -> !;
    fn usagefault_handler() -> !;
    fn svcall_handler() -> !;
    fn debugmonitor_handler() -> !;
    fn pendsv_handler() -> !;
    fn systick_handler() -> !;

    fn windowwatchdog_irq_handler() -> !;
    fn pvd_irq_handler() -> !;
    fn tamper_irq_handler() -> !;
    fn rtc_irq_handler() -> !;
    fn flash_irq_handler() -> !;
    fn rcc_irq_handler() -> !;
    fn exti0_irq_handler() -> !;
    fn exti1_irq_handler() -> !;
    fn exti2_irq_handler() -> !;
    fn exti3_irq_handler() -> !;
    fn exti4_irq_handler() -> !;
    fn dma1_channel1_irq_handler() -> !;
    fn dma1_channel2_irq_handler() -> !;
    fn dma1_channel3_irq_handler() -> !;
    fn dma1_channel4_irq_handler() -> !;
    fn dma1_channel5_irq_handler() -> !;
    fn dma1_channel6_irq_handler() -> !;
    fn dma1_channel7_irq_handler() -> !;
    fn adc1_2_irq_handler() -> !;
    fn usb_hp_can_tx_irq_handler() -> !;
    fn usb_lp_can_rx0_irq_handler() -> !;
    fn can_rx1_irq_handler() -> !;
    fn can_sce_irq_handler() -> !;
    fn exti9_5_irq_handler() -> !;
    fn tim1_brk_irq_handler() -> !;
    fn tim1_up_irq_handler() -> !;
    fn tim1_trg_com_irq_handler() -> !;
    fn tim1_cc_irq_handler() -> !;
    fn tim2_irq_handler() -> !;
    fn tim3_irq_handler() -> !;
    fn tim4_irq_handler() -> !;
    fn i2c1_ev_irq_handler() -> !;
    fn i2c1_er_irq_handler() -> !;
    fn i2c2_ev_irq_handler() -> !;
    fn i2c2_er_irq_handler() -> !;
    fn spi1_irq_handler() -> !;
    fn spi2_irq_handler() -> !;
    fn usart1_irq_handler() -> !;
    fn usart2_irq_handler() -> !;
    fn usart3_irq_handler() -> !;
    fn exti15_10_irq_handler() -> !;
    fn rtc_alarm_irq_handler() -> !;
    fn otg_fs_wkup_irq_handler() -> !;
    fn tim8_brk_tim12_irq_handler() -> !;
    fn tim8_up_tim13_irq_handler() -> !;
    fn tim8_trg_com_tim14_irq_handler() -> !;
    fn tim8_cc_irq_handler() -> !;
    fn dma1_stream7_irq_handler() -> !;
    fn fsmc_irq_handler() -> !;
    fn sdio_irq_handler() -> !;
    fn tim5_irq_handler() -> !;
    fn spi3_irq_handler() -> !;
    fn uart4_irq_handler() -> !;
    fn uart5_irq_handler() -> !;
    fn tim6_dac_irq_handler() -> !;
    fn tim7_irq_handler() -> !;
    fn dma2_stream0_irq_handler() -> !;
    fn dma2_stream1_irq_handler() -> !;
    fn dma2_stream2_irq_handler() -> !;
    fn dma2_stream3_irq_handler() -> !;
    fn dma2_stream4_irq_handler() -> !;
    fn eth_irq_handler() -> !;
    fn eth_wkup_irq_handler() -> !;
    fn can2_tx_irq_handler() -> !;
    fn can2_rx0_irq_handler() -> !;
    fn can2_rx1_irq_handler() -> !;
    fn can2_sce_irq_handler() -> !;
    fn otg_fs_irq_handler() -> !;
    fn dma2_stream5_irq_handler() -> !;
    fn dma2_stream6_irq_handler() -> !;
    fn dma2_stream7_irq_handler() -> !;
    fn usart6_irq_handler() -> !;
    fn i2c3_ev_irq_handler() -> !;
    fn i2c3_er_irq_handler() -> !;
    fn otg_hs_ep1_out_irq_handler() -> !;
    fn otg_hs_ep1_in_irq_handler() -> !;
    fn otg_hs_wkup_irq_handler() -> !;
    fn otg_hs_irq_handler() -> !;
    fn dcmi_irq_handler() -> !;
    fn default_irq_handler() -> !;
    fn hash_rng_irq_handler() -> !;
    fn fpu_irq_handler() -> !;
}

#[unsafe(no_mangle)]
pub extern "C" fn default_handler() -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn hardfault_handler() -> ! {
    loop {}
}

unsafe extern "C" {
    fn main() -> !;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reset_handler() -> ! {
    unsafe {
        // zeroing .bss
        let mut bss_ptr = &raw mut _sbss as *mut u32;
        let bss_end = &raw mut _ebss as *mut u32;
        while bss_ptr < bss_end {
            ptr::write_volatile(bss_ptr, 0);
            bss_ptr = bss_ptr.offset(1);
        }
        // copy data from flash to ram
        let data_start = &raw mut _sdata as *mut u32;
        let data_end = &raw mut _edata as *mut u32;
        let mut src = &_sidata as *const u32;

        let mut dest = data_start;
        while dest < data_end {
            ptr::write_volatile(dest, *src);
            dest = dest.wrapping_add(1);
            src = src.wrapping_add(1);
        }
    }

    unsafe {
        main();
    }
}

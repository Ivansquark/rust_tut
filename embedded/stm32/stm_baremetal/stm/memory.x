EXTERN(default_handler);
PROVIDE(nmi_handler = default_handler);
PROVIDE(hardfault_handler = default_handler);
PROVIDE(memorymanagement_handler = default_handler);
PROVIDE(busfault_handler = default_handler);
PROVIDE(usagefault_handler = default_handler);
PROVIDE(svcall_handler = default_handler);
PROVIDE(debugmonitor_handler = default_handler);
PROVIDE(pendsv_handler = default_handler);
PROVIDE(systick_handler = default_handler);

PROVIDE(windowwatchdog_irq_handler = default_handler);
PROVIDE(pvd_irq_handler = default_handler);
PROVIDE(tamper_irq_handler = default_handler);
PROVIDE(rtc_irq_handler = default_handler);
PROVIDE(flash_irq_handler = default_handler);
PROVIDE(rcc_irq_handler = default_handler);
PROVIDE(exti0_irq_handler = default_handler);
PROVIDE(exti1_irq_handler = default_handler);
PROVIDE(exti2_irq_handler = default_handler);
PROVIDE(exti3_irq_handler = default_handler);
PROVIDE(exti4_irq_handler = default_handler);
PROVIDE(dma1_channel1_irq_handler = default_handler);
PROVIDE(dma1_channel2_irq_handler = default_handler);
PROVIDE(dma1_channel3_irq_handler = default_handler);
PROVIDE(dma1_channel4_irq_handler = default_handler);
PROVIDE(dma1_channel5_irq_handler = default_handler);
PROVIDE(dma1_channel6_irq_handler = default_handler);
PROVIDE(dma1_channel7_irq_handler = default_handler);
PROVIDE(adc1_2_irq_handler = default_handler);
PROVIDE(usb_hp_can_tx_irq_handler = default_handler);
PROVIDE(usb_lp_can_rx0_irq_handler = default_handler);
PROVIDE(can_rx1_irq_handler = default_handler);
PROVIDE(can_sce_irq_handler = default_handler);
PROVIDE(exti9_5_irq_handler = default_handler);
PROVIDE(tim1_brk_irq_handler = default_handler);
PROVIDE(tim1_up_irq_handler = default_handler);
PROVIDE(tim1_trg_com_irq_handler = default_handler);
PROVIDE(tim1_cc_irq_handler = default_handler);
PROVIDE(tim2_irq_handler = default_handler);
PROVIDE(tim3_irq_handler = default_handler);
PROVIDE(tim4_irq_handler = default_handler);
PROVIDE(i2c1_ev_irq_handler = default_handler);
PROVIDE(i2c1_er_irq_handler = default_handler);
PROVIDE(i2c2_ev_irq_handler = default_handler);
PROVIDE(i2c2_er_irq_handler = default_handler);
PROVIDE(spi1_irq_handler = default_handler);
PROVIDE(spi2_irq_handler = default_handler);
PROVIDE(usart1_irq_handler = default_handler);
PROVIDE(usart2_irq_handler = default_handler);
PROVIDE(usart3_irq_handler = default_handler);
PROVIDE(exti15_10_irq_handler = default_handler);
PROVIDE(rtc_alarm_irq_handler = default_handler);
PROVIDE(otg_fs_wkup_irq_handler = default_handler);
PROVIDE(tim8_brk_tim12_irq_handler = default_handler);
PROVIDE(tim8_up_tim13_irq_handler = default_handler);
PROVIDE(tim8_trg_com_tim14_irq_handler = default_handler);
PROVIDE(tim8_cc_irq_handler = default_handler);
PROVIDE(dma1_stream7_irq_handler = default_handler);
PROVIDE(fsmc_irq_handler = default_handler);
PROVIDE(sdio_irq_handler = default_handler);
PROVIDE(tim5_irq_handler = default_handler);
PROVIDE(spi3_irq_handler = default_handler);
PROVIDE(uart4_irq_handler = default_handler);
PROVIDE(uart5_irq_handler = default_handler);
PROVIDE(tim6_dac_irq_handler = default_handler);
PROVIDE(tim7_irq_handler = default_handler);
PROVIDE(dma2_stream0_irq_handler = default_handler);
PROVIDE(dma2_stream1_irq_handler = default_handler);
PROVIDE(dma2_stream2_irq_handler = default_handler);
PROVIDE(dma2_stream3_irq_handler = default_handler);
PROVIDE(dma2_stream4_irq_handler = default_handler);
PROVIDE(eth_irq_handler = default_handler);
PROVIDE(eth_wkup_irq_handler = default_handler);
PROVIDE(can2_tx_irq_handler = default_handler);
PROVIDE(can2_rx0_irq_handler = default_handler);
PROVIDE(can2_rx1_irq_handler = default_handler);
PROVIDE(can2_sce_irq_handler = default_handler);
PROVIDE(otg_fs_irq_handler = default_handler);
PROVIDE(dma2_stream5_irq_handler = default_handler);
PROVIDE(dma2_stream6_irq_handler = default_handler);
PROVIDE(dma2_stream7_irq_handler = default_handler);
PROVIDE(usart6_irq_handler = default_handler);
PROVIDE(i2c3_ev_irq_handler = default_handler);
PROVIDE(i2c3_er_irq_handler = default_handler);
PROVIDE(otg_hs_ep1_out_irq_handler = default_handler);
PROVIDE(otg_hs_ep1_in_irq_handler = default_handler);
PROVIDE(otg_hs_wkup_irq_handler = default_handler);
PROVIDE(otg_hs_irq_handler = default_handler);
PROVIDE(dcmi_irq_handler = default_handler);
PROVIDE(default_irq_handler = default_handler);
PROVIDE(hash_rng_irq_handler = default_handler);
PROVIDE(fpu_irq_handler = default_handler);
PROVIDE(fpu_irq_handler = default_handler);

MEMORY
{
    FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 1024K
    RAM (xrw) : ORIGIN = 0x20000000, LENGTH = 128K
}

ENTRY(Reset_Handler)

_stack_size = 0x100;

SECTIONS
{
    /*
    .stack_position :
    {
        . = ALIGN(4);
        KEEP(*(.stack_position))
        LONG(_estack);
        . = ALIGN(4);
    } > FLASH
    */
    .isr_vector :
    {
        /* LONG(ORIGIN(RAM) + _stack_size); */
        LONG(_estack);
        . = ALIGN(4);
        KEEP(*(.isr_vector))
        . = ALIGN(4);
    } > FLASH

    .text :
    {
        . = ALIGN(4);
        *(.text)           
        *(.text*)          
        *(.glue_7)         
        *(.glue_7t)        
        *(.eh_frame)

        KEEP (*(.init))
        KEEP (*(.fini))

        . = ALIGN(4);
        _etext = .;        
    } > FLASH

    .rodata :
    {
        . = ALIGN(4);
        *(.rodata)         
        *(.rodata*)
        . = ALIGN(4);
    } > FLASH

    _sidata = LOADADDR(.data);

    .data : 
    {
        . = ALIGN(4);
        _sdata = .;        
        *(.data)           
        *(.data*)

        . = ALIGN(4);
        _edata = .;        
    } > RAM AT> FLASH

    ._stack (NOLOAD) :
    {
        . = ALIGN(8);
        . = . + _stack_size;
        . = ALIGN(8);
        _estack = .;       
    } > RAM

    .bss :
    {
        . = ALIGN(4);
        _sbss = .;         
        *(.bss)
        *(.bss*)
        *(COMMON)

        . = ALIGN(4);
        _ebss = .;         
    } > RAM


    /DISCARD/ :
    {
        *(.ARM.exidx*)
        *(.gnu.linkonce.armexidx.*)
    }
}

OUTPUT_ARCH(riscv)         /*体系结构*/

BASE_ADDRESS = 0x80200000; /*入口地址*/

ENTRY(_start)              /*入口*/
/* 内存布局 */
SECTIONS
{
    . = BASE_ADDRESS;
    _skernel = .;

    /*stext = .;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }*/
    .text : ALIGN(4K) {
        stext = .;
        *(.text.entry)
        *(.text .text.*)
        . = ALIGN(4K);
        etext = .;
    }

    srodata = .;
    .rodata : ALIGN(4K) {
        *(.rodata .rodata.*)
    }

    . = ALIGN(4K);
    erodata = .;

    /* sdata = .; */
    /* .data : { */
        /* *(.data .data.*) */
    /* } */

    .data : ALIGN(4K) {
        sdata = .;
        /* *(.data.boot_page_table) */
        /* . = ALIGN(4K); */
        *(.data .data.*)
        /* *(.sdata .sdata.*) */
        /* *(.got .got.*) */
    }

    . = ALIGN(4K);
    edata = .;


    .bss : AT(.) ALIGN(4K) {
        boot_stack = .;
        *(.bss.stack)
        . = ALIGN(4K);
        boot_stack_top = .;

        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        /* *(COMMON) */
        . = ALIGN(4K);
        ebss = .;
    }

    /* .bss : { */
        /* *(.bss.stack) */
        /* sbss = .; */
        /* *(.bss .bss.*) */
    /* } */

    /* . = ALIGN(4K); */
    /* ebss = .; */
    /* ekernel = .; */

    /DISCARD/ : {
        *(.eh_frame)
    }
}

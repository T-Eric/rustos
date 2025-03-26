# os/src/entry.asm
    .section .text.entry
    .globl _start
    .align 2
_start:
    la sp, boot_stack_top
    call boot_init
    mret
    call rust_main

    .section .bss.stack
    .globl boot_stack_lower_bound
boot_stack_lower_bound:
    .space 4096 * 16
    .globl boot_stack_top
boot_stack_top:

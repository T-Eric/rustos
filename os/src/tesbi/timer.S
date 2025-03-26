# all the interrupt handlers are defined here, or more
    .section .text.interrupt
    .globl _clock_interrupt_handler
    .align 2
_clock_interrupt_handler:
    csrrw sp, mscratch, sp
    sd t0, 0(sp)
    sd t1, 8(sp)
    sd t2, 16(sp)

    # setup next timer trigger
    ld t0, 24(sp) # address of mtimercmp
    ld t1, 32(sp) # timer interval
    ld t2, 0(t0) # current time
    add t2, t2, t1 # new time
    sd t2, 0(t0) # set newtime

    # setup timer interrupt for supervisor
    li t0, 2
    csrw sip, t0
    # csrrs zero, mip, t0

    # restore registers
    ld t0, 0(sp)
    ld t1, 8(sp)
    ld t2, 16(sp)
    csrrw sp, mscratch, sp
    mret
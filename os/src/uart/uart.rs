use bitflags::bitflags;

const MMIO_BASE:usize=0x10000000;

// DLAB=0
struct ReadPort{
  rbr:u8,
  ier:u8,//interrupt enable
  iir:u8,
  lcr:u8,
  mcr:u8,
  lsr:u8,
  msr:u8,
  scr:u8,
}

struct WritePort{
  thr:u8,
  ier:u8,
  fcr:u8,
  lcr:u8,
  mcr:u8,
  lsr:u8,
  _padding:u8, // not used
  scr:u8,
}

// uart control regs, where each bit can be set
bitflags! {
  struct InterruptEnable:u8{
    const RECV_AVIL=1<<0;
    const TRANS_EMPTY=1<<1;
    const ERRORED=1<<2;
    const STATUS_CHANGE=1<<3;
  }

  struct FifoControl:u8{
    const EN=1<<0;
    const CLR_RECV_FIFO=1<<1;
    const CLR_TRANS_FIFO=1<<2;
    //Select DMA mode, reserved, reserved
    const TWIG_14=0b11<<6;
  }

  struct LineControl:u8{
    const DATA_8=0b11;
    const DLAB_EN=1<<7;
  }

  struct ModemControl:u8{
    const DATA_TERMINAL_READY=1<<0;
    const AUX_OUTPUT_2=1<<3;
  }
  
  struct LineStatus:u8{
    const INPUT_AVIL=1<<0;
    const OUTPUT_EMPTY=1<<5;
  }
}


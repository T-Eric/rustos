use bitflags::bitflags;
use core::hint::spin_loop;
use core::sync::atomic::{AtomicPtr, Ordering};
use lazy_static::lazy_static;

macro_rules! wait_for {
    ($cond:expr) => {
        while !$cond {
            spin_loop();
        }
    };
}

bitflags! {
  struct LineStatus:u8{
    const INPUT_AVAILABLE=1<<0;
    const OUTPUT_EMPTY=1<<5;
  }
}

// Read, DLAB=0
// 不要直接用数u8，否则读写要转换指针涉及太多unsafe操作
// 但是AtomicPtr似乎不能与bitflags的类兼容
#[repr(C)]
pub struct SerialPort {
    rbr: AtomicPtr<u8>,
    ier: AtomicPtr<u8>,
    fcr: AtomicPtr<u8>,
    lcr: AtomicPtr<u8>,
    mcr: AtomicPtr<u8>,
    lsr: AtomicPtr<u8>,
    msr: AtomicPtr<u8>,
    scr: AtomicPtr<u8>,
}

impl SerialPort {
    pub unsafe fn new(base: usize) -> Self {
        let base = base as *mut u8;
        Self {
            rbr: AtomicPtr::new(base),
            ier: AtomicPtr::new(base.add(1)),
            fcr: AtomicPtr::new(base.add(2)),
            lcr: AtomicPtr::new(base.add(3)),
            mcr: AtomicPtr::new(base.add(4)),
            lsr: AtomicPtr::new(base.add(5)),
            msr: AtomicPtr::new(base.add(6)),
            scr: AtomicPtr::new(base.add(7)),
        }
    }

    pub fn init(&mut self) {
        let rbr = self.rbr.load(Ordering::Relaxed);
        let ier = self.ier.load(Ordering::Relaxed);
        let fcr = self.fcr.load(Ordering::Relaxed);
        let lcr = self.lcr.load(Ordering::Relaxed);
        let mcr = self.mcr.load(Ordering::Relaxed);
        unsafe {
            // disable interrupts
            ier.write_volatile(0x00);

            // enable DLAB
            lcr.write_volatile(0x80);

            // LSB/MSB baud rate 38.4k
            rbr.write_volatile(0x03);
            ier.write_volatile(0x00);

            // disable DLAB, set align 3 with no parity
            lcr.write_volatile(0x03);

            // enable FIFO, clear queues, set interrupt 14bytes
            fcr.write_volatile(0xc7);

            // not knowing what to do
            mcr.write_volatile(0x0b);

            // enable interrupts
            ier.write_volatile(0x01);
        }
    }

    pub fn putchar(&self, data: u8) {
        let rbr = self.rbr.load(Ordering::Relaxed);
        let lsr = self.lsr.load(Ordering::Relaxed);
        unsafe {
            let status = LineStatus::from_bits_truncate(*lsr); // 不知道是否要每次刷新？

            // BS=8,DEL=0x7f
            match data {
                0x08 | 0x7f => {
                    // 我也不知道为啥要这么做
                    wait_for!(status.contains(LineStatus::OUTPUT_EMPTY));
                    rbr.write_volatile(0x08);
                    wait_for!(status.contains(LineStatus::OUTPUT_EMPTY));
                    rbr.write_volatile(b' ');
                    wait_for!(status.contains(LineStatus::OUTPUT_EMPTY));
                    rbr.write_volatile(0x08);
                }
                _ => {
                    wait_for!(status.contains(LineStatus::OUTPUT_EMPTY));
                    rbr.write_volatile(data);
                }
            }
        }
    }

    pub fn _getchar(&mut self) -> u8 {
        let rbr = self.rbr.load(Ordering::Relaxed);
        let lsr = self.lsr.load(Ordering::Relaxed);
        unsafe {
            let status = LineStatus::from_bits_truncate(*lsr); // 不知道是否要每次刷新？
            wait_for!(status.contains(LineStatus::INPUT_AVAILABLE));
            rbr.read_volatile()
        }
    }
}

// 创造全局静态uart实例
lazy_static! {
    pub static ref UART: SerialPort = {
        let mut uart = unsafe { SerialPort::new(crate::tesbi::VIRT_UART0_ADDRESS) };
        uart.init();
        uart
    };
}

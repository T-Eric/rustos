use crate::tesbi::sbi::shutdown;
use crate::Log;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println_info!(
            Log::Panic,
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message()
        )
    } else {
        println_info!(Log::Panic, "Panicked: {}", info.message())
    }
    shutdown();
}

use crate::sbi::shutdown;
use crate::Log;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println_info!(
            Log::Error,
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().as_str().unwrap()
        )
    } else {
        println_info!(Log::Error,"Panicked: {}", info.message().as_str().unwrap())
    }
    shutdown(true);
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
  let err = info.message();
  if let Some(location) = info.location() {
    println!(
      "Panicked at {}:{}, {}",
      location.file(),
      location.line(),
      err
    );
  } else {
    println!("Panicked: {}", err);
  }
  loop {}
}
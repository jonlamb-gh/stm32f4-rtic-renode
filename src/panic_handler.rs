// TODO do something like quartiq/stabilizer's panic_handler
// https://github.com/quartiq/stabilizer/blob/488cc2171b4c32ad516f68094c3b5cd91b663597/src/hardware/mod.rs#L81

use core::{
    panic::PanicInfo,
    sync::atomic::{compiler_fence, Ordering::SeqCst},
};

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use cortex_m::interrupt;

    interrupt::disable();

    let w = unsafe { crate::logger::get_logger() };
    writeln!(w, "\n********************************").ok();
    writeln!(w, "PANIC").ok();
    writeln!(w, "{info}").ok();
    writeln!(w, "********************************").ok();

    loop {
        compiler_fence(SeqCst);
    }
}

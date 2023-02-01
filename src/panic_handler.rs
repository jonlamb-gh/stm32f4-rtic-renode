// TODO - once log impl is moved to a task, consider forging USART3 here

use core::{
    panic::PanicInfo,
    sync::atomic::{compiler_fence, Ordering::SeqCst},
};

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use cortex_m::interrupt;

    interrupt::disable();

    log::error!("{info}");

    loop {
        compiler_fence(SeqCst);
    }
}

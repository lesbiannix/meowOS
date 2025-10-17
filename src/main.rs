#![no_std]
#![no_main]

use core::panic::PanicInfo;
use meow_os::{hlt_loop, println};

/// Custom panic handler: prints panic info and halts the CPU.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n================== KERNEL PANIC ==================");

    if let Some(location) = info.location() {
        println!("Panic at {}:{}", location.file(), location.line());
    } else {
        println!("Panic location unknown.");
    }

    println!("Message: {}", info.message());

    println!("================== KERNEL PANIC ==================\n");

    #[cfg(feature = "allocator_debug")]
    {
        use meow_os::allocator::MAPPED_PAGES;
        let mapped = MAPPED_PAGES.lock();
        println!("Mapped pages (for debugging):");
        for (page, frame) in mapped.iter() {
            println!("Page {:>#018x} -> Frame {:>#018x}", page.as_u64(), frame);
        }
    }

    hlt_loop()
}

/// Entry point of the kernel
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize GDT, IDT, PICs, and enable interrupts
    meow_os::init();

    // Initialize heap here if needed
    // meow_os::allocator::init_heap(...);

    println!("Kernel initialized successfully!");

    hlt_loop()
}


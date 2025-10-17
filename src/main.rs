#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use meow_os::{hlt_loop, println};

entry_point!(kernel_main);
use meow_os::memory;
use x86_64::VirtAddr;


use meow_os::memory;
use x86_64::VirtAddr;

use core::panic::PanicInfo;

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // Initialize GDT, IDT, PICs, and enable interrupts
    meow_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // Initialize heap
    meow_os::allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    println!("Kernel initialized successfully!");

    hlt_loop()
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

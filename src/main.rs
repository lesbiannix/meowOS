#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]

extern crate alloc;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use meowOS::{println, hlt_loop};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use meowOS::memory::BootInfoFrameAllocator;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    
    // Initialize GDT and interrupts
    println!("Initializing GDT and interrupts...");
    meowOS::init();
    println!("GDT and interrupts initialized!");

    // Initialize memory management
    println!("Initializing memory management...");
    println!("Physical memory offset: {:#x}", boot_info.physical_memory_offset);
    
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { 
        println!("Creating mapper...");
        meowOS::memory::init(phys_mem_offset) 
    };
    println!("Mapper created!");
    
    let mut frame_allocator = unsafe {
        println!("Creating frame allocator...");
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    println!("Frame allocator created!");

    // Initialize heap
    println!("Initializing heap...");
    match meowOS::allocator::init_heap(&mut mapper, &mut frame_allocator) {
        Ok(_) => println!("Heap initialized successfully!"),
        Err(e) => {
            println!("Failed to initialize heap: {:?}", e);
            println!("This likely means we ran out of frames or couldn't map pages");
            hlt_loop();
        }
    }

    println!("Memory management initialized!");

    // Test heap allocation
    println!("Testing heap allocation...");
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    use alloc::string::String;

    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..100 {
        vec.push(i);
    }
    println!("vec at {:p}, len: {}", vec.as_slice(), vec.len());

    let reference_string = String::from("Hello from the heap!");
    println!("{}", reference_string);

    println!("All heap tests passed!");

    // Start the shell
    println!("Starting shell...");
    let mut shell = meowOS::shell::Shell::new();
    shell.run();

    println!("It did not crash!");
    hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

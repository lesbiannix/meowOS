# Memory Management for meowOS

This memory management system is based on Phil Oppermann's "Writing an OS in Rust" tutorial series and provides comprehensive memory management for your kernel.

## Components

### 1. **memory.rs** - Core Memory Management
- `init()`: Initializes the page table mapper
- `BootInfoFrameAllocator`: Allocates physical frames from bootloader memory map
- `translate_addr()`: Translates virtual addresses to physical addresses
- Page table management utilities

### 2. **allocator.rs** - Heap Allocator Framework
- Heap initialization
- Global allocator setup
- Multiple allocator implementations

### 3. **allocator/bump.rs** - Bump Allocator
- Simple linear allocator
- Fast allocation, no deallocation support (except when all freed)
- Good for temporary allocations

### 4. **allocator/linked_list.rs** - Linked List Allocator
- Full allocation and deallocation support
- Uses freed memory blocks as linked list nodes
- General-purpose allocator

### 5. **allocator/fixed_size_block.rs** - Fixed-Size Block Allocator
- Pre-defined block sizes for common allocations
- Fast allocation/deallocation
- Combines fixed blocks with fallback allocator
- Best performance for production use

## Integration Steps

### Step 1: Update Cargo.toml

Add these dependencies:

```toml
[dependencies]
bootloader = "0.9"
x86_64 = "0.14"
spin = "0.9"
linked_list_allocator = "0.10"

[dependencies.lazy_static]
version = "1.4"
features = ["spin_no_std"]
```

### Step 2: Enable Required Features

Add to the top of your `src/lib.rs`:

```rust
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
```

### Step 3: Add Memory Modules

Place the provided files in your project:
- `src/memory.rs`
- `src/allocator.rs`
- `src/allocator/bump.rs`
- `src/allocator/linked_list.rs`
- `src/allocator/fixed_size_block.rs`

### Step 4: Declare Modules in lib.rs

```rust
extern crate alloc;

pub mod memory;
pub mod allocator;
```

### Step 5: Add Alloc Error Handler

Add this to your `src/lib.rs`:

```rust
use alloc::alloc::Layout;

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
```

### Step 6: Initialize Memory in main.rs

Update your `kernel_main` function:

```rust
use blog_os::memory;
use blog_os::allocator;
use bootloader::BootInfo;
use x86_64::VirtAddr;

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use blog_os::memory::{self, BootInfoFrameAllocator};
    
    println!("Hello meowOS!");
    
    // Initialize interrupts, GDT, etc.
    blog_os::init();
    
    // Initialize memory management
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    
    // Initialize heap
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");
    
    println!("Memory initialized successfully!");
    
    // Now you can use heap allocations!
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    use alloc::string::String;
    
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);
    
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());
    
    let string = String::from("Hello from the heap!");
    println!("{}", string);
    
    // Your shell and other kernel code...

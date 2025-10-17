use crate::println;
use heapless::spsc::Queue;
use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;

static SCANCODE_QUEUE: Mutex<Queue<u8, 100>> = Mutex::new(Queue::new());

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(
            ScancodeSet1::new(),
            layouts::Us104Key,
            HandleControl::Ignore
        ));
}

pub fn add_scancode(scancode: u8) {
    if SCANCODE_QUEUE.lock().enqueue(scancode).is_err() {
        println!("WARNING: scancode queue full; dropping keyboard input");
    }
}

pub fn read_char() -> char {
    let mut keyboard = KEYBOARD.lock();
    loop {
        if let Some(scancode) = SCANCODE_QUEUE.lock().dequeue() {
            if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
                if let Some(key) = keyboard.process_keyevent(key_event) {
                    match key {
                        DecodedKey::Unicode(character) => return character,
                        _ => {}
                    }
                }
            }
        }
    }
}

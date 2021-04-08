#![no_std]
#![feature(extended_key_value_attributes)]
#![feature(alloc_error_handler)]
extern crate alloc;

use rust_drivers::*;

extern crate freertos_rs;
use freertos_rs::*;

#[repr(u8)]
enum c_void {
    __variant1,
    __variant2,
}

extern {
    fn pvPortMalloc(size: u32) -> *mut c_void;
    fn vPortFree(p: *mut c_void);	
}

#[alloc_error_handler]
fn foo(_: core::alloc::Layout) -> ! {
    panic!("OOM!");
}

use core::alloc::{GlobalAlloc, Layout};

pub struct FreeRtosAllocator;

unsafe impl GlobalAlloc for FreeRtosAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        pvPortMalloc(layout.size() as u32) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        vPortFree(ptr as *mut c_void)
    }
}

#[global_allocator]
static GLOBAL: FreeRtosAllocator = FreeRtosAllocator;


#[no_mangle]
pub extern "C" fn rustExampleInit(_info: *mut DeckInfo) {
    Task::new().name("hello").stack_size(128).start(|| {
        CurrentTask::delay(Duration::ms(1000));
        console_print!(b"Hello from rustExampleDriver init\n\0".as_ptr() as *const cty::c_char);
    }).unwrap();
}

#[no_mangle]
pub extern "C" fn rustExampleTest() -> bool {
    console_print!(b"Hello from rustExampleDriver test\n\0".as_ptr() as *const cty::c_char);
    true
}

#[no_mangle]
static rust_example: RustDeckDriver = RustDeckDriver {
    name: DriverName(b"rustExampleDriver\0".as_ptr() as *const cty::c_char),
    init: rustExampleInit,
    test: rustExampleTest,
    usedPeriph: 0,
    usedGpio: 0,
    requiredEstimator: 0,
    requiredLowInterferenceRadioMode: false,
    vid: 0xDE,
    pid: 0xAD,
    memoryDef: ConstDeckMemDef(core::ptr::null() as *const DeckMemDef_t),
};

deck_driver!(rust_example, RustExampleDriver);

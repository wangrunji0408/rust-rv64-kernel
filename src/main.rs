#![no_std]
#![feature(start)]
#![feature(asm)]

use core::panic::PanicInfo;

mod sbi;

const VIRTUAL_START: usize = 0xffffffff_c0000000;
const PHYSICAL_START: usize = 0x80000000;
const KERNEL_OFFSET: usize = VIRTUAL_START - PHYSICAL_START;

#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _start() -> ! {
    #[repr(align(0x1000))]
    struct PageTable {
        entries: [u64; 512],
    }

    static mut ROOT_PAGE_TABLE: PageTable = PageTable {
        entries: [0; 512]
    };

    let root_page_table_paddr = ROOT_PAGE_TABLE.entries.as_ptr() as usize - KERNEL_OFFSET;
    let root_page_table = &mut *(root_page_table_paddr as *mut PageTable);
    let idx0 = (PHYSICAL_START >> 12 >> 9 >> 9) & 0o777;
    let idx1 = (VIRTUAL_START >> 12 >> 9 >> 9) & 0o777;
    let pte = (PHYSICAL_START >> 12 << 10) | 0xf; // VRWX
    root_page_table.entries[idx0] = pte as u64;
    root_page_table.entries[idx1] = pte as u64;

    const SV39: usize = 8 << 60;
    let satp = (root_page_table_paddr >> 12) | SV39;

    asm!("csrw satp, $0; sfence.vma" :: "r"(satp) :: "volatile");
    asm!("add sp, sp, $0; jalr $1" :: "r"(KERNEL_OFFSET), "r"(main as usize) :: "volatile");
    loop {}
}

extern "C" fn main() {
    print("hello world!"); 
}

fn print(s: &str) {
    for &c in s.as_bytes() {
        sbi::console_putchar(c as usize); 
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn abort() -> ! {
    panic!("abort");
}

// Entry point for this program.
#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    0
}

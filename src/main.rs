#![no_std]
#![no_main]

mod vga_buffer;

// mod vga_buffer;
    use core::panic::PanicInfo;

    static HELLO: &[u8] = b"Hello World!";

    #[no_mangle]
    pub extern "C" fn _start() -> ! {
        use core::fmt::Write;
        // let vga_buffer = 0xb8000 as *mut u8;
        //
        // // for (i, &byte) in HELLO.iter().enumerate() {
        // //     unsafe  {
        // //         *vga_buffer.offset(i as isize * 2) = byte;
        // //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        // //     }
        // // }
        //
        // "Hello, world!"
        //     .as_bytes()
        //     .iter()
        //     .flat_map(|bt| [*bt, 0xc as u8])
        //     .enumerate()
        //     .for_each(|(i, byte)| unsafe {
        //         *vga_buffer.offset(i as isize) = byte;
        //     });

        // vga_buffer::print_something();

        vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
        write!(vga_buffer::WRITER.lock(), " , some numbers: {} {}", 42, 1.337).unwrap();

        loop {}
    }

    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! {
        loop {}
    }

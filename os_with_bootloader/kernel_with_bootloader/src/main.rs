#![no_std]
#![no_main]

mod writer;

use bootloader_api::config::Mapping;
use writer::FrameBufferWriter;

// #[macro_use]

use x86_64::instructions::hlt;

//Use the entry_point macro to register the entry point function: bootloader_api::entry_point!(kernel_main)
//optionally pass a custom config
pub static BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};

bootloader_api::entry_point!(my_entry_point, config = &BOOTLOADER_CONFIG);

fn my_entry_point(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    let frame_buffer_info = boot_info.framebuffer.as_mut().unwrap().info(); //info

    let buffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut(); //framebuffer

    // let x_pos = 10;
    // let y_pos = 20;

    let mut frame_buffer_writer = FrameBufferWriter::new(buffer, frame_buffer_info);

    //frame_buffer_writer.set_position(180, 200);
    // use core::fmt::Display; //below requires this

    // writeln!(frame_buffer_writer,"Testing testing {} and {}",  1, 10.0 / 2.0).unwrap();

    macro_rules! print {
        ($($arg:tt)*) => ({
            use core::fmt::Write;
           // let mut printer = frame_buffer_writer; //the printer, is basically the frame_buffer_writer
            let _ = write!(frame_buffer_writer, $($arg)*);
        });
    } //only difference as it is!
      //this print! macro basically is "coded" to execute what writeln! ought to do, just that in this case, the print!
      //hides the declaration of frame_buffer_writer that would be ordinarily declared when using writeln as seen above

      print!("We continue !! Thank you {}Hallo\n", "God!!!");
      print!("\n");
      print!("\n");
      print!("\n");
      print!("\n");
      frame_buffer_writer.set_position(200, 350); //the framebufferwriter using the setposition function updates the x_pos, y_pos values from it's default 0,0 as seen in writer.rs
      print!("We continue !! Thank you {}Hallo", "God!!!");
      print!("\n");
      print!("\n");
      print!("\n");
      print!("\n");
      frame_buffer_writer.set_position(300, 150);
      print!("DONE!!!!!!!!!!!! Thank you {}Hallo", "God!!!");
    //   print!("We continue !! Thank you {}Hallo", "God!!!");
    // print!("We continue !! Thank you {}", "God!!! \nHallo");

    loop {
        hlt(); //stop x86_64 from being unnecessarily busy while looping
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        hlt();
    }
}

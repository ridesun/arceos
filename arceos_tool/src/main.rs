use core::mem::size_of;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

#[repr(C)]
pub struct AppHeader {
    magic: u64,
    app_count: u64,
    app_size: [u64; 6],
}

impl AppHeader {
    pub const MAGIC: u64 = 0x4150505F;
}
fn main() {
    let mut app_data1 = Vec::new();
    // let mut app_data2 = Vec::new();
    File::open("hello_app.bin")
        .unwrap()
        .read_to_end(&mut app_data1)
        .unwrap();
    // File::open("app2.bin")
    //     .unwrap()
    //     .read_to_end(&mut app_data2)
    //     .unwrap();
    let mut app_size = [0; 6];
    app_size[0] = app_data1.len() as u64;
    println!("{}", app_size[0]);
    // app_size[1]=app_data1.len() as u64;
    let header = AppHeader {
        magic: AppHeader::MAGIC,
        app_size,
        app_count: 1,
    };

    let mut output = File::create("apps.bin").unwrap();

    let header_bytes = unsafe {
        std::slice::from_raw_parts(&header as *const _ as *const u8, size_of::<AppHeader>())
    };
    output.write_all(header_bytes).unwrap();

    // output.write_all(&app_data2).unwrap();
    output.write_all(&app_data1).unwrap();

    let total_size = 32 * 1024 * 1024; // 32MB
    output.seek(SeekFrom::Start(total_size - 1)).unwrap();
    output.write_all(&[0]).unwrap();
}

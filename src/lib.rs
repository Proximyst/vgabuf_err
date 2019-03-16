#![feature(decl_macro)]
#![no_std]

/// Writes "ERR: $error_code" onto the VGA buffer in
/// white text on red background.
pub macro error($error_code:expr) {{
    let error_code: char = $error_code;
    let buf = unsafe { &mut *(0xb8000 as *mut [[u32; 80]; 25]) };

    // Every byte must be manually set, because
    // this can be called by any code in any mode
    // with any form for stack, including none.
    //
    // Because this supports no stack, it also
    // has to be a macro.

    buf[0][0] = 0x524F_454F;
    buf[0][1] = 0x3A4F_524F;
    buf[0][2] = 0x204F_004F | ((error_code as u8 as u32) << 8);
}}

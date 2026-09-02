use std::io::Write;

// chunking protocol
pub fn write_sequence<W: Write>(handle: &mut W, chunks: &[String], format_code: u8, width: Option<u32>, height: Option<u32>) {
    for (i, chunk) in chunks.iter().enumerate(){
        let is_last = i == chunks.len() - 1;
        let more_flag = if is_last { 0 } else { 1 };

        if i == 0 {
            let control = match (width,height) {
                (Some(w),Some(h)) => {
                    format!("f={},a=T,s={},v={},m={}",format_code,w,h,more_flag)
                }
                _ => format!("f={},a=T,m={}",format_code,more_flag),
            };
            write!(handle, "\x1b_G{};{}\x1b\\", control, chunk).unwrap();
        } else {
            write!(handle, "\x1b_Gm={};{}\x1b\\", more_flag, chunk).unwrap()
        }
    }

    handle.flush().unwrap();
}

pub fn write_animation_first_frame<W: Write>(handle: &mut W, chunks: &[String], format_code: u8, width: u32,height: u32,image_id: u32) {
    for (i,chunk) in chunks.iter().enumerate()  {
        let is_last = i == chunks.len() - 1;
        let more_flag = if is_last { 0 } else { 1 };

        if i == 0 {
            write!(handle, "\x1b_Gf={},a=T,s={},v={},i={},q=1,m={};{}\x1b\\",format_code,width,height,image_id,more_flag,chunk).unwrap();
        } else {
            write!(handle, "\x1b_Gm={};{}\x1b\\",more_flag,chunk).unwrap();
        }
    }

    handle.flush().unwrap();
}

pub fn write_animation<W:Write>(handle: &mut W, chunks: &[String], format_code: u8,width: u32,height: u32, image_id: u32, delay: u32) {
    for (i,chunk) in chunks.iter().enumerate()  {
        let is_last = i == chunks.len() - 1;
        let more_flag = if is_last { 0 } else { 1 };
       
       if i == 0 {
           write!(handle, "\x1b_Ga=f,f={},s={},v={},i={},z={},m={};{}\x1b\\", format_code,width,height, image_id, delay, more_flag, chunk ).unwrap();
       } else {
           write!(handle, "\x1b_Ga=f,m={};{}\x1b\\",more_flag,chunk).unwrap();
       }
    }
    handle.flush().unwrap();
}

pub fn set_frame_delay<W: Write>(handle: &mut W, delay: u32, image_id: u32) {
    write!(handle, "\x1b_Ga=a,i={},r=1,z={}\x1b\\",image_id,delay).unwrap();
    handle.flush().unwrap();
}

pub fn start_animation<W:Write>(handle: &mut W, image_id: u32) {
    write!(handle, "\x1b_Ga=a,i={},s=3,v=1\x1b\\",image_id).unwrap();
    handle.flush().unwrap();
}
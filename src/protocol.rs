use std::io::Write;

// chunking protocol
pub fn write_sequence<W: Write>(handle: &mut W, chucks: &[String], format_code: u8, width: Option<u32>, height: Option<u32>,) {
    for (i, chunk) in chucks.iter().enumerate(){
        let is_last = i == chucks.len() - 1;
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
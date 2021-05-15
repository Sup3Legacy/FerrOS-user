
use alloc::string::String;
use alloc::vec::Vec;

pub fn decompose(string: String) -> Result<Vec<String>, ()> {
    let mut vector = Vec::new();
    let mut curr_str = String::new();
    let mut pos = 0;
    while pos < string.len() {
        match string.as_bytes()[pos] {
            b' ' => {
                if !curr_str.is_empty() {
                    vector.push(curr_str);
                    curr_str = String::new();
                };
                pos += 1;
            },

            b'\'' => {
                if !curr_str.is_empty() {
                    vector.push(curr_str);
                    curr_str = String::new();
                }
                if let Ok((curr_str, pos2)) = get_char(&string, pos + 1, b'\'') {
                    vector.push(curr_str);
                    pos = pos2;
                } else {
                    return Err(())
                }
            },

            b'"' => {
                if !curr_str.is_empty() {
                    vector.push(curr_str);
                    curr_str = String::new();
                }
                if let Ok((curr_str, pos2)) = get_char(&string, pos + 1, b'"') {
                    vector.push(curr_str);
                    pos = pos2;
                } else {
                    return Err(())
                }
            },

            c => {
                curr_str.push(c as char);
                pos += 1;
            }
        }
    }
    if !curr_str.is_empty() {
        vector.push(curr_str)
    }
    Ok(vector)
}

fn get_char(s_in: &String, mut pos: usize, searched: u8) -> Result<(String, usize), ()> {
    let mut s_out = String::new();
    while pos < s_in.len() {
        if s_in.as_bytes()[pos] == searched {
            return Ok((s_out, pos + 1))
        } else {
            s_out.push(s_in.as_bytes()[pos] as char);
            pos += 1;
        }
    }
    Err(())
}
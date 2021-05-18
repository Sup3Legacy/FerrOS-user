use alloc::collections::BTreeMap;
use alloc::string::String;

use ferr_os_librust::io;

pub fn main(string: &String, env: &BTreeMap<String, String>) -> Result<String, ()> {
    let mut pos = 0;
    let mut out = String::new();
    while pos < string.len() {
        match string.as_bytes()[pos] {
            b'$' => match parse_var(string, pos + 1, env) {
                Err(()) => return Err(()),
                Ok((new_pos, text)) => {
                    pos = new_pos;
                    out.push_str(&text);
                }
            },
            b'\'' => {
                out.push('\'');
                pos += 1;
                while pos < string.len() && string.as_bytes()[pos] != b'\'' {
                    out.push(string.as_bytes()[pos] as char);
                    pos += 1;
                }
                if pos == string.len() {
                    return Err(());
                }
                out.push('\'');
                pos += 1
            }
            c => {
                out.push(c as char);
                pos += 1;
            }
        }
    }
    Ok(out)
}

fn parse_var(
    string: &String,
    pos: usize,
    env: &BTreeMap<String, String>,
) -> Result<(usize, String), ()> {
    if string.len() <= pos {
        Err(())
    } else if string.as_bytes()[pos] == b'(' {
        if string.len() == pos + 1 {
            Err(())
        } else if string.as_bytes()[pos + 1] == b'(' {
            io::_print(&String::from("Computation is not implemented\n"));
            Err(())
        } else {
            let mut name = String::new();
            for c in string.bytes().skip(pos + 1) {
                if c == b')' {
                    return Ok((pos + 2 + name.len(), get_var(&name, env)));
                } else {
                    name.push(c as char)
                }
            }
            Err(())
        }
    } else {
        let s = String::from(string.as_bytes()[pos] as char);
        Ok((pos+1, get_var(&s, env)))
    }
}


fn get_var(name: &String, env: &BTreeMap<String, String>) -> String {
    match env.get(name) {
        None => String::from(""),
        Some(v) => String::from(v),
    }
}
mod hextable;
use hextable::hextable;
use std::str::from_utf8;

pub fn encode(inp: String) -> String {
    let byt: Vec<u8> = inp.into_bytes();
    let mut outbytes: Vec<u8> = vec![];
    for b in byt{
        match b {
            0x20|48..=57|65..=89|97..=122 => outbytes.push(b),
            37 => {
                outbytes.push(b);
                outbytes.push(b);
            }
            _ => {
                let hext= hextable(b);
                outbytes.push('%' as u8);
                outbytes.push(hext[0]);
                outbytes.push(hext[1]);
            }
        }
    }
    from_utf8(&outbytes).unwrap().to_string()
}

pub fn decode(inp: String) -> Result<String, &'static str> {
    let byt: Vec<u8> = inp.into_bytes();
    let mut state: i32 = 0;
    let mut outstr_u8: Vec<u8> = vec![];
    let mut convstr: String = "".to_string();
    for b in byt{
        match state {
            0 => {
                match b {
                    0x20|48..=57|65..=89|97..=122 => outstr_u8.push(b),
                    b'%' => state = 1, // '%' is escape character here
                    _ => { 
                        println!("Illegal character {} in decode stream", b);
                        return Err("Illegal character in decode stream")
                    }
                }
            }
            1 => {
                if u8::is_ascii_hexdigit(&b) { 
                    convstr.push(char::from(b)); 
                    state = 2;
                }
                else if b == b'%' {
                    outstr_u8.push(b);
                    state = 0;
                }
                else { return Err("Illegal character in hex decode 2"); }
            }
            2 => {
                if u8::is_ascii_hexdigit(&b) { convstr.push(char::from(b)); }
                else { return Err("Illegal character in hex decode 3"); }

                let t = u8::from_str_radix(&convstr, 16);
                match t {
                    Ok(c) => outstr_u8.push(c),
                    Err(_) => return Err("Failed to convert hex in decode"),
                }
                state = 0;
                convstr = "".to_string();
            }
            _ => return Err("Illegal state in percent_decode"),
        }
    }
    Ok(from_utf8(&outstr_u8).unwrap().to_string())
}

#[cfg(test)]
mod tests {
    use crate::{encode, decode};
    #[test]
    fn it_works() {

//test encode,  decode
        let test1 = r#"dsrich@hot über mail.com"#.to_string();
        let test2 = r#"dsrichZ40hot Zc3Zbcber mailZ2ecom"#.to_string();
        let test3 = r#"dsrichZZ40hot ZZc3ZZbcber mailZZ2ecom"#.to_string();
        let test4 = r#"\"fu*t[]{}~ ü!@#$%^&*()_+`-=|"#.to_string();
        assert_eq!(encode(test1.clone()), test2.clone());
        assert_eq!(decode(encode(test4.clone())).unwrap(), test4.clone());
        assert_eq!(encode(test2.clone()), test3.clone());
        assert_eq!(encode(encode(test1.clone())), test3.clone());
        assert_eq!(decode(encode(test1.clone())).unwrap(), test1.clone());
        assert_eq!(decode(decode(encode(encode(test1.clone()))).unwrap()).unwrap(), test1.clone());
        assert_eq!(decode(encode(test2.clone())).unwrap(), test2.clone());
        assert_eq!(decode(encode(test4.clone())).unwrap(), test4.clone());
        assert_eq!(decode(test2.clone()).unwrap(), test1.clone());        
        assert_eq!(decode(test3.clone()).unwrap(), test2);
        assert_eq!(decode(decode(test3).unwrap()).unwrap(), test1);
    }
}
pub struct Report<'a> {
    line: u32,
    cur_char: &'a str,
    msg: &'a str,
}

impl<'a> Report<'a> {
    pub fn new(line: u32, cur_char: &'a str, msg: &'a str) {
        let err_format = format!("[line {line}] Error {cur_char}: {msg}");
        panic!("{err_format}");
    }
}


extern crate kana;

fn main() {
    let r = std::io::stdin();

    let buff = &mut String::new();
    while r.read_line(buff).is_ok() && 0 != buff.len() {
        print!("{}", kana::ascii2wide(buff));
        buff.clear();
    }
}

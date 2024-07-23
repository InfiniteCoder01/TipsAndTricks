// Simple parser to parse floating-point numbers from a stream

pub fn parse_float(num: &mut f64, base: &mut Option<f64>, ch: char) {
    if ch == '.' {
        *base = Some(0.1);
    } else {
        if let Some(base) = base {
            *num += (ch as u32 - '0' as u32) as f64 * *base;
            *base /= 10.0
        } else {
            *num *= 10.0;
            *num += (ch as u32 - '0' as u32) as f64;
        }
    }
}

pub fn parse(s: &str) -> f64 {
    let mut num = 0.0;
    let mut base = None;
    for ch in s.chars() {
        parse_float(&mut num, &mut base, ch);
    }
    num
}

pub fn main() {
    println!("{}", parse("42"));
    println!("{}", parse("4.2"));
    println!("{}", parse(".42"));
}

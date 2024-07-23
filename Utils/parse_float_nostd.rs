// Simple parser to parse floating-point numbers without std

pub fn parse(s: &str) -> f64 {
    let mut base = 0.1;
    for ch in s.chars() {
        if ch == '.' {
            break;
        }
        base *= 10.0;
    }
    let mut total = 0.0;
    for ch in s.chars() {
        if ch.is_digit(10) {
            total += (ch as u32 - '0' as u32) as f64 * base;
            base /= 10.0;
        }
    }
    total
}

pub fn main() {
    println!("{}", parse("42"));
    println!("{}", parse("4.2"));
    println!("{}", parse(".42"));
}

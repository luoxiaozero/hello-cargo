fn add(a: u8, b: u8) -> u8 {
    a + b
}

macro_rules! add {
    ($a: expr, $b: expr) => {
        add($a, $b)
    };
}

fn main() {
    let b = add!(10, 9);
    println!("{}", b);
}

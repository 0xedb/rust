fn main() {
    let info: (&str, u32, usize, f64) = ("bruno", 100, 0xedb, 302.32);
    let array = [0xedb; 10];

    println!("{} {} {} {}", info.0, info.2, info.1, info.3);
    println!("{}\n{}", array[3], array.len());
    echo(30, 3.0);
    println!("{}", ten());
    println!("{}", hundered());
}

fn echo(x: i32, y: f64) {
    println!("HELLO {}--{}", x, y);
}

fn ten() -> i64 {
    10
}

fn hundered() -> i128 {
    100
}

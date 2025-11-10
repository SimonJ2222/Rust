fn print_sum(a: i32, b: i32) {
    println!("Summe: {}", a + b);
}

fn main() {
    let d1 = 2.5;
    let d2 = 3.5;

    print_sum(d1, d2); // Kompilerfehler: expected `i32`, found `f64`
    //print_sum(d1 as i32, d2 as i32);
}

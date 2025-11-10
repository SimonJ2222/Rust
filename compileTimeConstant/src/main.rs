fn main() {
    const SECONDS_PER_HOUR: u32 = 60 * 60; //Berechnung zur Compile-Time
    const VALUE: u32 = square(5); //Berechnung zur Compile-Time

    println!("Eine Stunde hat {} Sekunden.", SECONDS_PER_HOUR);
    println!("5² = {}", VALUE);
}

const fn square(x: u32) -> u32 {
    x * x
}



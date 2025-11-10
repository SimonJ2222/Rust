fn create_number() -> &i32 {
    let x = 42;
    &x //Kompiliert nicht
}

fn main() {
    let ptr = create_number();
    println!("{}", ptr);
}

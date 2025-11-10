use std::thread;

fn main() {
    let mut counter = 0;

    let t1 = thread::spawn(|| {
        for _ in 0..1_000_000 {
            counter += 1; 
        }
    });

    let t2 = thread::spawn(|| {
        for _ in 0..1_000_000 {
            counter += 1; 
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Counter: {}", counter);
} //Kompilerfehler

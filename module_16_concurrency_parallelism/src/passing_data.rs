use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn function1(count: usize) {
    println!("In function 1");
    for i in 0..count {
        println!("ping");
    }
    println!("function 1 done");
}

fn main() {
    let count = 10;

    let f1 = move || {
        println!("In closure f1");
        function1(count);
    };

    println!("Right before spawn");
    let thread1 = thread::spawn(f1);

    // main is now waiting for thread.
    thread1.join().unwrap();
}

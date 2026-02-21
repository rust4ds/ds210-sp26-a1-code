fn secondfunction(x: i32) -> i32 {
    println!("number is {}", x);
    x % 3
}

fn myfunction(num: i32) {
    if secondfunction(num) == 0 {
        println!("All done!");
        return;
    } else if secondfunction(num) == 1 {
        myfunction(num + 2);
    } else {
        myfunction(num - 1);
    }
}

fn main() {
    myfunction(5); // starting value
}

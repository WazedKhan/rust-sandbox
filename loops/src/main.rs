fn main() {
    let mut count:u32 = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };
    println!("The result is: {result}");
}


fn main() {
    println!("Hello, world!");
    another_function();
    let y = {
        let x = 6;
        x + 1
    };

    println!("The value of y is: {y}");
    five();
}

fn five() -> i32{
    5 + 1
}

fn another_function() {
    println!("Hello from another functions!");
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: u32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

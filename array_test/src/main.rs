use std::io;
fn main() {
    let a = [1, 2, 3, 4, 5];

    let b = &[-10i32;32][1..3];

    println!("Please enter an array index.{:?}",b);

    println!("Please enter an array index.");

    let mut index = String::new();
    // 读取控制台的输出
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}


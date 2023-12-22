#[derive(Debug)]
enum Test{
    A(i32),
    B(f32),
    C(Vec<i32>),
}

fn main() {
    println!("Hello, world!");
    let mut vec_test = Vec::new();
    vec_test.push(32);
    vec_test.push(22);
    let  test = Test::C(vec_test);
    println!("{:?}",test);
}

use std::{slice::from_raw_parts,str::from_utf8_unchecked};

fn get_memory_location()->(usize,usize){
    let string = "Hello World!";
    let pointer = string.as_ptr() as usize;
    let length = string.len();
    (pointer,length)
}

fn get_str_at_locationn(pointer:usize,length:usize) -> &'static str{
    unsafe {from_utf8_unchecked(from_raw_parts(pointer as *const u8,length))}
}
fn main(){
    let (pointer,length) = get_memory_location();
    let message = get_str_at_locationn(pointer,length);
    println!("The {} bytes at 0x{:X} stored:{}",length,pointer,message);
    let _messagea = get_str_at_locationn(1000,10);

    let a:Box<i32> = Box::new(10);
    let b:*const i32 = &*a;
    let c:*const i32 = Box::into_raw(a);
    unsafe{
        println!("value b is {}",*b);
        println!("value c is {}",*c);
    }
    let s1: Box<str> = "Hello there!".into();
    let s2: Box<str> = Box::new("Hello there!" as str);
    println!(" box value b is {}",s1);
}
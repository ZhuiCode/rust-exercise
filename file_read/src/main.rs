use std::io;
use std::fs::File;
use std::io::{BufReader, BufRead};
fn main() -> io::Result<()>{
    let res = File::open("./test.param")?;
    let mut reader = BufReader::new(res);
    let mut buffer = String::new();
    
    let rdsize = reader.read_line(&mut buffer)?;
    println!("{rdsize}");  
    //let width = 100;
    //println!("{:width$}",a=3);
    let width = 5;
    println!("Hello {:width$}!", format!("Hello {:<5}!", "x"));

    println!("{}", format!("{:+}", 27));
    //format!("Hello {:<5}!", "x");

    
    Ok(())
}

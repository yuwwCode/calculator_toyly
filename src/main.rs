#![allow(unused)]
mod utils;

type Calculate = utils::calculate::Calculate;
use std::env;
fn main() {
    //let args: Vec<String> = env::args().collect();
    //println!("{:?},1 as u8={}", args,'1' as u8);
    println!("put in the Formula");
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("read_line error!");

        if input == "exit" {
            println!("exited!");
            break;
        }
        let c1 = Calculate::new_from_string(input);
        let res = c1.start();
        match res {
            Ok(o) => {
                match o.get_num() {
                    utils::number::Data::F(f) => println!("= {}", f),
                    utils::number::Data::I(i) => println!("= {}", i),
                };
            }
            Err(e) => {
                println!("error : {}", e);
            }
        }
        println!("");
        println!("put in anything to continue:");
    }
}

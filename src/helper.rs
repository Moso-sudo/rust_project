use std::{io, u32};
use std::cmp::Ordering;
use rand::Rng;
use colored::*;
pub fn helper(){

println!("Guessing game");
   
let secret_number = rand ::thread_rng().gen_range(1,101);
println!("the secret number is: {}", secret_number);
println!("Enter your guess");
loop{
    println!("please input your guess");
    let mut guess =String::new();
    io::stdin()
    .read_line( &mut guess)
    .expect("failed to read line");
let guess : u32 = match guess.trim().parse(){
    Ok(num)=>num,
    Err(_)=>continue,
    
};
println!("you guessed: {}", guess);
 match guess.cmp(&secret_number){
    Ordering::Equal=>{
        println!("{}","you win".green());
        break;
    }
    Ordering::Greater=>println!("{}","too big!".red()),
    Ordering::Less=>println!( "{}", "Too small".red()),      
}
}

}

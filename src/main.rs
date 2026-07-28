use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // numero random

    let mut guess = String::new(); // inizializa una variable mutable

    println!("The secret number is: {secret_number}"); // imprime el numero secreto
    
    loop {
        
        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)  //metodo para leer una entrada de usuario
            .expect("Failed to read line"); // en caso que falla el metodo read_line, se imprime el mensaje de error

        let guess: u32 = guess.trim().parse().expect("Please type a number!"); // convierte la variable inpu user a un i32

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {   // realiza una comparacion entre el numero secreto y el numero ingresado por el usuario
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
    }



    }
    

}
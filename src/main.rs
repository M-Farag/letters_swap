use std::io;

fn main() {

    let mut user_input:String = String::new();
    println!("Please write the letters to swap ex: x -> y ?!");
    io::stdin().read_line(&mut user_input).expect("Err Reading your input");
    let letters = letters_to_swap(user_input.trim());
    println!("You will swap {}, with {}",letters.0,letters.1);

    println!("Please input the text you wanna process ");
    let mut user_input:String = String::new();
    io::stdin().read_line(&mut user_input).expect("Err reading your input");
    let user_input:String = process_text(user_input.trim(), letters);

    println!("Modified text is:\n {}",user_input);
}

fn letters_to_swap(user_input:&str) ->(char,char)
{
    (
    user_input.chars().take(1).last().unwrap(),
    user_input.chars().take(3).last().unwrap()
    )
}

fn process_text(user_input:&str, letters_to_swap:(char,char)) -> String
{
    let mut new_text = String::with_capacity(user_input.len());
    for mut bit in user_input.chars() {
        if bit == letters_to_swap.0 {
            bit = letters_to_swap.1;
        }
        new_text.push(bit)
    }

    new_text
}

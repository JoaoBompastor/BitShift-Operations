use std::io;

fn main() -> (){
    let number: i32 = read_number().unwrap();
    let shift: i32 = read_number().unwrap();
    println!("{} in binary: {:08b} -> {:08b} = {}", number, number, right_shift(number as u8, shift as u8), right_shift(number as u8, shift as u8));

    let bit_to_read: u8 = read_number().unwrap() as u8;
    println!("bit {} is {}", bit_to_read, read_bit(number as u8, bit_to_read));

    for bit in 0..=number - 1{
        println!("{}", read_bit(number as u8, bit as u8));
    }
}

fn read_number() -> Result<i32, std::io::Error> {
    let mut input: String = String::new();
    println!("Enter an integer: ");
    io::stdin().read_line(&mut input).expect("Error reading line");
    let num: i32 = input.trim().parse().expect("Invalid input");
    Ok(num)
}

fn right_shift(number: u8, shift: u8) -> u8 {
    return number << shift as u8;
}

fn read_bit(word: u8, bit_to_read: u8) ->  u8 {
    return word & right_shift(word, bit_to_read)
}

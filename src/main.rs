use std::io;

fn main() -> (){
    println!("{}, {:08b} => {:08b} {}", 1, 1, 1 << 3, 1 << 3);
    println!("{}", 222 & (1 << 3));
    for num in 0..=10 {
        println!("o primeiro bit do numero {:08b} => {}", num, read_bit(num, 1))
    }

    println!("{:08b}", invert_bit(1u8, 0u8));
}

fn read_number(mensagem: &str) -> Result<i32, std::io::Error> {
    let mut input: String = String::new();
    println!("{}", mensagem);
    io::stdin().read_line(&mut input).expect("Error reading line");
    let num: i32 = input.trim().parse().expect("Invalid input");
    Ok(num)
}

fn read_bit(word: u8, bit_to_read: u8) ->  u8 {
    (word >> bit_to_read) & 1
}

fn invert_bit(word: u8, bit_to_flip: u8) -> u8 {
    word ^ (1 << bit_to_flip)
}

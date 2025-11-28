fn main() -> (){
    println!("{}", right_shift(1, 1));
    println!("{}", right_shift(1, 2));
    println!("{}", right_shift(1, 3));
}


fn right_shift(number: u8, shift: u8) -> u8 {
    return number << shift as u8;
}

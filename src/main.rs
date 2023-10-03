
fn get_rem_euclid_m() -> i32{
    return ('z' as u8 - 'a' as u8) as i32;
}

fn shift_character(c: char, n: i32) -> char{
    let c_as_number = (c as u8 - 'a' as u8) as i32;
    let m = get_rem_euclid_m();
    let shifted_c_as_number = (c_as_number + n).rem_euclid(m);
    return (shifted_c_as_number + 'a' as u8 as i32) as u8 as char;
}

fn shift_encode(text: &String, n: i32) -> String{
    let res = text.chars().into_iter()
        .map(|c| shift_character(c, n).to_string())
        .reduce(|acc, c| format!("{}{}", acc, c)).unwrap();

    return res;
}


fn ex_1(){

    let plaintext = "attackatonce".to_string();
    println!("Encryption");
    println!("Plaintext: {plaintext}");

    let cyphertext = shift_encode(&plaintext, 3);
    println!("Cyphertext: {cyphertext}");

    println!("Decryption");
    let decyphered = shift_encode(&cyphertext, -3);
    println!("Plaintext: {decyphered}");
    println!("Cyphertext: {cyphertext}");
}


// fn ex_2(){
//     const M: i32 = 26;
//     let numbers_to_invert = [ 1, 3, 5, 7, 9, 11, 15,
//         17, 19, 21, 23, 25];
// }


fn main() {
    ex_1();

}

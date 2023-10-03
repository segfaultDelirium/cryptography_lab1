use std::fmt::format;

fn get_rem_euclid_m() -> u8{
    return 'z' as u8 - 'a' as u8 + 1;
}

fn shift_character(c: char, n: i32) -> char{
    let c_as_number = c as u8 - 'a' as u8;
    let m = get_rem_euclid_m();
    let shifted_c_as_number = (c_as_number as i32 + n).rem_euclid(m as i32) as u8;
    return (shifted_c_as_number + 'a' as u8) as char;
}

fn shift_encode(text: &String, n: i32) -> String{
    let res = text.chars().into_iter()
        .map(|c| shift_character(c, n).to_string())
        .reduce(|acc, c| format!("{}{}", acc, c)).unwrap();

    return res;
}


fn ex_1(){
    println!();
    println!("ex_1");
    let plaintext = "attackatonce".to_string();
    println!("Encryption");
    println!("Plaintext: {plaintext}");

    let cyphertext = shift_encode(&plaintext, 3);
    println!("Cyphertext: {}", cyphertext.to_uppercase());

    println!("Decryption");
    let decyphered = shift_encode(&cyphertext, -3);
    println!("Plaintext: {decyphered}");
    println!("Cyphertext: {}", cyphertext.to_uppercase());
}



fn invert_number_by_trial_and_error(j: &i32, n: i32) -> i32{
    let res = match (1 .. n).into_iter().find(|k| (j * k).rem_euclid(n) == 1 ) {
        Some(v) => v,
        None => {
            println!("count not found inverted number for {j} in modulo {n}.");
            0
        }

    };
    return res;
}

fn rozNWD(j: i32, k: i32) -> (i32, i32, i32){
    if j == 0 {return (k, 0 , 1)}

    let r = k.rem_euclid(j);
    let (d, x_prim, y_prim) = rozNWD(r, j);
    let x = y_prim - (k / j) * x_prim;
    let y = x_prim;

    return (d, x, y)
}

fn invert_by_euclid(j: &i32, n: i32) -> i32{

    let (_d, x ,_y) = rozNWD(j + 0, n);
    // println!("d: {d}, x: {x}, y: {y}");
    return if x >= 0 { x} else {x + n}
}

fn ex_2(){
    println!();
    println!("ex_2");

    const N: i32 = 26;
    let numbers_to_invert: Vec<i32> = [ 1, 3, 5, 7, 9, 11, 15,
        17, 19, 21, 23, 25].to_vec();

    let inverted_numbers = (&numbers_to_invert).into_iter()
        .map(|x| invert_number_by_trial_and_error(x, N)).collect::<Vec<i32>>();

    let pairs = numbers_to_invert.iter().zip(&inverted_numbers);
    pairs.for_each(|(x, inverted_x)| {
       println!("inv of {x} is {inverted_x}");
    });

    let inverted_numbers_by_euclid = (&numbers_to_invert).into_iter()
        .map(|x| invert_by_euclid(x, N)).collect::<Vec<i32>>();

    let pairs_euclid = numbers_to_invert.iter().zip(&inverted_numbers_by_euclid);
    pairs_euclid.for_each(|(x, inverted_x)| {
        println!("GCD_ext: inv of {x} is {inverted_x}");
    });
}

/// should convert character from 'a' to 'z' to number between 0 and 26
fn char_to_number(c: char) -> u8{
    c as u8 - 'a' as u8
}

/// should convert number between 0 and 26 to character from 'a' to 'z'
fn number_to_char(x: u8) -> char{
    (x + 'a' as u8) as char
}

fn affine_encode_char(c: char, a: i32, b: i32) -> char{
    let c_as_number = char_to_number(c) as i32;
    let m = get_rem_euclid_m() as i32;
    let encrypted_number = (a * c_as_number + b ).rem_euclid(m);
    return number_to_char(encrypted_number as u8);
}

fn affine_decode_char(c: char, inverted_a: i32, b: i32) -> char{
    let c_as_number = char_to_number(c) as i32;
    let m = get_rem_euclid_m() as i32;
    let decrypted_number = (inverted_a  * (c_as_number - b )).rem_euclid(m);
    return number_to_char(decrypted_number as u8);
}


fn affine_encode(text: &String, a: i32, b: i32) -> String{

    let res = text.chars().into_iter()
        .map(|c| affine_encode_char(c, a, b).to_string())
        .reduce(|acc, c| format!("{}{}", acc, c)).unwrap();

    return res;
}

fn affine_decode(text: &String, inverted_a: i32, b: i32) -> String{

    let res = text.chars().into_iter()
        .map(|c| affine_decode_char(c, inverted_a, b).to_string())
        .reduce(|acc, c| format!("{}{}", acc, c)).unwrap();

    return res;
}

fn ex_3(){
    println!();
    println!("ex_3");

    const a: i32 = 17;
    const b: i32 = 20;
    let n = get_rem_euclid_m() as i32;
    assert!(n == 26);
    let inverted_a = invert_by_euclid(&a, n);
    println!("a = {a}, a_inv = {inverted_a}");
    let plaintext = "attackatonce".to_string();
    println!("Encryption");
    println!("Plaintext: {plaintext}");

    let cyphertext = affine_encode(&plaintext, a, b);
    println!("Cyphertext: {}", cyphertext.to_uppercase());

    println!("Decryption");
    let decyphered = affine_decode(&cyphertext, inverted_a, b);
    println!("Plaintext: {decyphered}");
    println!("Cyphertext: {}", cyphertext.to_uppercase());
}

fn decode_vigenere(text: &String, key: &String) -> String{


    "".to_string()
}

fn repeat_string_n_times(s: &String, n: i32) -> String{
    let res = match (1..n)
        .map(|x| "".to_string())
        .reduce(|acc, _i| format!("{}{}", s, s)){
        Some(v) => v,
        None => s.clone()
    };
    return res;
}

fn repeat_string_to_length(s: &String, length: usize) -> String {

    let copy_times = length / s.len() + 1;
    let res: String = repeat_string_n_times(s, copy_times as i32).chars().take(length).collect();
    return res;
    // return "".to_string();
}

fn get_key_with_length(key: &String, length: usize) -> String{
    if key.len() == length {return key.clone()};
    if key.len() > length {return key.chars().take(length).collect::<String>()}
    if key.len() < length {return repeat_string_to_length(key, length)}

    // below case should never happen
    return "".to_string();
}


fn encode_vigenere(text: &String, key: &String) -> String{

    let new_key: Vec<i32> = get_key_with_length(key, text.len())
            .chars()
            .map(|c| char_to_number(c) as i32).collect();
    println!("new key: {:?}", new_key);

    let res: String = text.chars().zip(new_key)
        .map(|(c, k)| shift_character(c, k)).collect();

    // let res = text.chars().into_iter()
    //     .map(|c| )
    //     .reduce(|acc, c| format!("{}{}", acc, c)).unwrap();

    return res;
    // "".to_string()
}

fn ex_4(){
    println!();
    println!("ex_4");

    let key = "cipher".to_string();
    println!("key: {}", key.to_uppercase());

    println!("Encryption");
    let plaintext = "thiscryptosystemisnotsecure".to_string();
    println!("Plaintext: {plaintext}");

    let cyphertext = encode_vigenere(&plaintext, &key);
    println!("Cyphertext: {}", cyphertext.to_uppercase());

    println!("Decryption");
    let decyphered = decode_vigenere(&cyphertext, &key);
    println!("Plaintext: {decyphered}");
    println!("Cyphertext: {}", cyphertext.to_uppercase());
}

fn main() {
    ex_1();

    ex_2();
    ex_3();
    ex_4();
}

use std::fmt::{Debug, Display, Formatter, Pointer};
mod matrix;
// use crate::matrix::matrix;
mod encode_helpers;
use crate::encode_helpers::encode_helpers as encode;

fn shift_encode(text: &String, n: i32) -> String {
    let res = text
        .chars()
        .into_iter()
        .map(|c| encode::shift_character(&c, &n).to_string())
        .reduce(|acc, c| format!("{}{}", acc, c))
        .unwrap();

    return res;
}

fn ex_1() {
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

fn invert_number_by_trial_and_error(j: &i32, n: i32) -> i32 {
    let res = match (1..n).into_iter().find(|k| (j * k).rem_euclid(n) == 1) {
        Some(v) => v,
        None => {
            println!("count not found inverted number for {j} in modulo {n}.");
            0
        }
    };
    return res;
}

fn invert_by_euclid(j: &i32, n: i32) -> i32 {
    let (_d, x, _y) = encode::rozNWD(j + 0, n);
    // println!("d: {d}, x: {x}, y: {y}");
    return if x >= 0 { x } else { x + n };
}

fn ex_2() {
    println!();
    println!("ex_2");

    const N: i32 = 26;
    let numbers_to_invert: Vec<i32> = [1, 3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25].to_vec();

    let inverted_numbers = (&numbers_to_invert)
        .into_iter()
        .map(|x| invert_number_by_trial_and_error(x, N))
        .collect::<Vec<i32>>();

    let pairs = numbers_to_invert.iter().zip(&inverted_numbers);
    pairs.for_each(|(x, inverted_x)| {
        println!("inv of {x} is {inverted_x}");
    });

    let inverted_numbers_by_euclid = (&numbers_to_invert)
        .into_iter()
        .map(|x| invert_by_euclid(x, N))
        .collect::<Vec<i32>>();

    let pairs_euclid = numbers_to_invert.iter().zip(&inverted_numbers_by_euclid);
    pairs_euclid.for_each(|(x, inverted_x)| {
        println!("GCD_ext: inv of {x} is {inverted_x}");
    });
}

fn affine_encode_char(c: char, a: i32, b: i32) -> char {
    let c_as_number = encode::char_to_number(&c) as i32;
    let m = encode::get_rem_euclid_m() as i32;
    let encrypted_number = (a * c_as_number + b).rem_euclid(m);
    return encode::number_to_char(&(encrypted_number as u8));
}

fn affine_decode_char(c: char, inverted_a: i32, b: i32) -> char {
    let c_as_number = encode::char_to_number(&c) as i32;
    let m = encode::get_rem_euclid_m() as i32;
    let decrypted_number = (inverted_a * (c_as_number - b)).rem_euclid(m);
    return encode::number_to_char(&(decrypted_number as u8));
}

fn affine_encode(text: &String, a: i32, b: i32) -> String {
    let res = text
        .chars()
        .into_iter()
        .map(|c| affine_encode_char(c, a, b).to_string())
        .reduce(|acc, c| format!("{}{}", acc, c))
        .unwrap();

    return res;
}

fn affine_decode(text: &String, inverted_a: i32, b: i32) -> String {
    let res = text
        .chars()
        .into_iter()
        .map(|c| affine_decode_char(c, inverted_a, b).to_string())
        .reduce(|acc, c| format!("{}{}", acc, c))
        .unwrap();

    return res;
}

fn ex_3() {
    println!();
    println!("ex_3");

    const a: i32 = 17;
    const b: i32 = 20;
    let n = encode::get_rem_euclid_m() as i32;
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

fn repeat_string_n_times(s: &String, n: i32) -> String {
    let res = match (0..n)
        .map(|x| "".to_string())
        .reduce(|acc, _i| format!("{}{}", acc, s))
    {
        Some(v) => v,
        None => s.clone(),
    };
    return res;
}

fn repeat_string_to_length(s: &String, length: usize) -> String {
    let copy_times = 2 + length / s.len();
    let res: String = repeat_string_n_times(s, copy_times as i32)
        .chars()
        .take(length)
        .collect();
    return res;
    // return "".to_string();
}

fn get_key_with_length(key: &String, length: usize) -> String {
    if key.len() == length {
        return key.clone();
    };
    if key.len() > length {
        return key.chars().take(length).collect::<String>();
    }
    if key.len() < length {
        return repeat_string_to_length(key, length);
    }

    // below case should never happen
    return "".to_string();
}

fn encode_vigenere(text: &String, key: &String) -> String {
    let new_key: Vec<i32> = get_key_with_length(key, text.len())
        .chars()
        .map(|c| encode::char_to_number(&c) as i32)
        .collect();
    // let new_key_as_string = (&new_key).into_iter().map(|x| number_to_char((x + 0) as u8)).collect::<String>();
    // println!("new key: {:?}", new_key_as_string);

    let res: String = text
        .chars()
        .zip(new_key)
        .map(|(c, k)| encode::shift_character(&c, &k))
        .collect();

    return res;
}

fn decode_vigenere(text: &String, key: &String) -> String {
    let new_key: Vec<i32> = get_key_with_length(key, text.len())
        .chars()
        .map(|c| encode::char_to_number(&c) as i32)
        .collect();
    // let new_key_as_string = (&new_key).into_iter().map(|x| number_to_char((x + 0) as u8)).collect::<String>();
    // println!("new key: {:?}", new_key_as_string);

    let res: String = text
        .chars()
        .zip(new_key)
        .map(|(c, k)| encode::shift_character(&c, &(-1 * k)))
        .collect();

    return res;
}

fn ex_4() {
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

/// this assumes that text is 4 character long
// fn convert_text_to_matrix(text: &String) -> Array2<u8>{
//
//     let text_as_numbers = text.chars().map(|c| char_to_number(c)).collect::<Vec<u8>>();
//     let res: Array2<u8> = arr2(&[[text_as_numbers[0], text_as_numbers[1]],
//                                      [text_as_numbers[2], text_as_numbers[3]]]);
//     return res;
// }
//
// fn encode_hill(text: &String, key: &Array2<u8>) -> String {
//
//     let text_as_vector = convert_text_to_matrix(text);
//     println!("key: \n{:?}", key);
//     println!("text_as_vector: \n{:?}", text_as_vector);
//     let result_vector: Array2<u8> = key * text_as_vector;
//
//
//     let res2: String = result_vector.into_iter().map(|x| number_to_char(x.rem_euclid(get_rem_euclid_m())) ).collect();
//
//     return res2;
// }
//
// fn decode_hill(text: &String, key: &Array2<u8>) -> String {
//
//     "".to_string()
// }

struct Matrix2x2<T: Debug + Display> {
    m11: T,
    m12: T,
    m21: T,
    m22: T,
}

impl<T: Debug + Display> Debug for Matrix2x2<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}, {},\n {}, {}",
            self.m11, self.m12, self.m21, self.m22
        )
    }
}

impl<T: Debug + Display> Display for Matrix2x2<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n[{}, {},\n {}, {}]",
            self.m11, self.m12, self.m21, self.m22
        )
    }
}

impl<T: Debug + Display + Clone> Matrix2x2<T> {
    fn new(m11: T, m12: T, m21: T, m22: T) -> Matrix2x2<T> {
        Matrix2x2 { m11, m12, m21, m22 }
    }

    fn from_vec(vec: Vec<T>) -> Matrix2x2<T> {
        assert!(vec.len() >= 4);
        Matrix2x2::new(
            vec.get(0).unwrap().clone(),
            vec.get(1).unwrap().clone(),
            vec.get(2).unwrap().clone(),
            vec.get(3).unwrap().clone(),
        )
    }

    fn from_vec_rows(vec_rows: Vec<Vec<T>>) -> Matrix2x2<T> {
        assert!(vec_rows.len() >= 2);
        let row0 = vec_rows.get(0).unwrap();
        let row1 = vec_rows.get(1).unwrap();
        assert!(row0.len() >= 2);
        assert!(row1.len() >= 2);

        Matrix2x2::new(
            row0.get(0).unwrap().clone(),
            row0.get(1).unwrap().clone(),
            row1.get(0).unwrap().clone(),
            row1.get(1).unwrap().clone(),
        )
    }
}

fn ex_5() {
    println!();
    println!("ex_5");

    let key = [[11, 8], [3, 7]];
    let key_as_vec_of_vec = key
        .into_iter()
        .map(|x| x.into_iter().collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let sample_matrix = Matrix2x2::new(11, 8, 3, 7);
    let sample_matrix2 = Matrix2x2::from_vec(key.into_iter().flatten().collect::<Vec<i32>>());
    let sample_matrix3 = Matrix2x2::from_vec_rows(key_as_vec_of_vec);
    println!("sample_matrix: {}", sample_matrix);
    println!("sample_matrix2: {}", sample_matrix2);
    println!("sample_matrix3: {}", sample_matrix3);

    // let inverse_key = key2.try_inverse().unwrap();

    // println!("Encryption");
    // let plaintext = "july".to_string();
    // println!("Plaintext: {plaintext}");
    //
    // let cyphertext = encode_hill(&plaintext, &key);
    // println!("Cyphertext: {}", cyphertext.to_uppercase());
    //
    // println!("Decryption");
    // let decyphered = decode_hill(&cyphertext, &key);
    // println!("Plaintext: {decyphered}");
    // println!("Cyphertext: {}", cyphertext.to_uppercase());
}

fn main() {
    ex_1();

    ex_2();
    ex_3();
    ex_4();
    ex_5();
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_repeat_string_n_times() {
        let input_string = "testing".to_string();
        let input_n_times = 4;

        let expected = "testingtestingtesting".to_string();

        let result = repeat_string_n_times(&input_string, input_n_times);
        assert_eq!(expected, result);
    }
}

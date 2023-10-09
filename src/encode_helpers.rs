pub mod encode_helpers {

    /// should convert character from 'a'..='z' to number between 0..=25
    pub fn char_to_number(c: &char) -> u8 {
        *c as u8 - 'a' as u8
    }

    /// should convert number between 0 and 26 to character from 'a' to 'z'
    pub fn number_to_char(x: &u8) -> char {
        (x + 'a' as u8) as char
    }

    /// will simply return 26
    pub fn get_rem_euclid_m() -> u8 {
        return 'z' as u8 - 'a' as u8 + 1;
    }

    /// will convert c to number 0..=25 and shift it by n modulo 26
    /// if n > 0 it will shift to the right, if n < 0 it will shift to the left
    pub fn shift_character(c: &char, n: &i32) -> char {
        let c_as_number = *c as u8 - 'a' as u8;
        let m = get_rem_euclid_m();
        let shifted_c_as_number = (c_as_number as i32 + n).rem_euclid(m as i32) as u8;
        return (shifted_c_as_number + 'a' as u8) as char;
    }

    /// will calculate greatest common divisor using euclid algorithm
    pub fn rozNWD(j: i32, k: i32) -> (i32, i32, i32) {
        if j == 0 {
            return (k, 0, 1);
        }

        let r = k.rem_euclid(j);
        let (d, x_prim, y_prim) = rozNWD(r, j);
        let x = y_prim - (k / j) * x_prim;
        let y = x_prim;

        return (d, x, y);
    }
}

#[cfg(test)]
mod tests {
    use crate::encode_helpers::encode_helpers::{
        char_to_number, get_rem_euclid_m, number_to_char, shift_character,
    };
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_char_to_number() {
        let input = ('a'..='z').collect::<Vec<char>>();
        let expected = (0..=25).collect::<Vec<u8>>();

        let result = input.iter().map(|c| char_to_number(c)).collect::<Vec<u8>>();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_number_to_char() {
        let input = (0..=25).collect::<Vec<u8>>();
        let expected = ('a'..='z').collect::<Vec<char>>();

        let result = input
            .iter()
            .map(|x| number_to_char(x))
            .collect::<Vec<char>>();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_rem_euclid_m() {
        let expected = 26;
        let result = get_rem_euclid_m();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_shift_character_right() {
        let input = ('a'..='z').collect::<Vec<char>>();
        let shift_by = 3;

        let expected = ('d'..='z').chain('a'..'d').collect::<Vec<char>>();
        let result = input
            .iter()
            .map(|c| shift_character(c, &shift_by))
            .collect::<Vec<char>>();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_shift_character_left() {
        let input = ('a'..='z').collect::<Vec<char>>();
        let shift_by = -3;

        let expected = ('x'..='z').chain('a'..'x').collect::<Vec<char>>();
        let result = input
            .iter()
            .map(|c| shift_character(c, &shift_by))
            .collect::<Vec<char>>();

        assert_eq!(result, expected);
    }
}

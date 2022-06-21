use std::collections::HashMap;
use crate::algorithms::Cipher;
use crate::utils::Key;

/// Przechowuje dodatkowe infromacje potrzebne dla szyfru wieloalfabetowego (Szyfr Vigenère'a)
pub struct PolyalphabeticData(Key);

/// Metody dodatkowe związane z szyfrem wieloalfabetowym (Szyfr Vigenère'a)
impl PolyalphabeticData {
    /// Tworzy nową strukturę z informacjami szyfru wieloalfabetowego (Szyfr Vigenère'a)
    /// 
    /// # Arguments
    /// 
    /// * `code` - klucz do wykorzystania (musi spełniać warunki[^1])
    /// 
    /// # Errors
    /// 
    /// * [^1]Klucz musi posiadać znaki alfabetyczne ascii (a-z + A-Z)
    pub fn new(code: Key) -> Result<PolyalphabeticData, Box<dyn std::error::Error>> {
        if code.data.iter().any(|s: &String| s.chars().any(|c: char| !c.is_alphabetic())) {
            return Err("Invalid code! Valid code must contain ASCII alphabetic characters".into())
        }
        Ok(PolyalphabeticData(code))
    }
}

/// Implementacja metod szyfrujących
impl Cipher for PolyalphabeticData {
    fn encrypt(&self, data: &str) -> String { 
        let ascii: HashMap<char, usize> = ('A'..='Z').chain('a'..='z')
                                                    .enumerate()
                                                    .map(|(index, sign): (usize, char)| (sign, index))
                                                    .collect();

        data.chars().zip(self.0.data.iter().cycle()).map(|(sign, vec): (char, &String)|{
            if sign.is_ascii_alphabetic() {
                let code_char: char = vec.chars().nth(0).unwrap();

                let data_num: usize = ascii[&sign];
                let code_num: usize = ascii[&code_char];
                let result: usize = (data_num + code_num) % ascii.len();
                *ascii.iter().find(|(_k, &v): &(&char, &usize)| v == result).unwrap().0
            } else {
                sign
            }
        }).collect::<String>() 
    }
    fn decrypt(&self, data: &str) -> String { 
        let ascii: HashMap<char, usize> = ('A'..='Z').chain('a'..='z')
                                                    .enumerate()
                                                    .map(|(index, sign): (usize, char)| (sign, index))
                                                    .collect();

        data.chars().zip(self.0.data.iter().cycle()).map(|(sign, vec): (char, &String)|{
            if sign.is_ascii_alphabetic() {
                let code_char: char = vec.chars().nth(0).unwrap();

                let data_num: i8 = ascii[&sign] as i8;
                let code_num: i8 = ascii[&code_char] as i8;
                let mut result: i8 = data_num - code_num;
                if result < 0 { 
                    result = ascii.len() as i8 + result; 
                }
                let result = result as usize % ascii.len();
                *ascii.iter().find(|(_k, &v): &(&char, &usize)| v == result).unwrap().0
            } else {
                sign
            }
        }).collect::<String>() 
    }
}
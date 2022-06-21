use crate::algorithms::Cipher;
use crate::utils::Key;
use rand::{Rng, prelude::ThreadRng};

/// Przechowuje dodatkowe infromacje potrzebne dla szyfru homofonicznego
pub struct HomophonicData(Key);

/// Metody dodatkowe związane z szyfrem homofonicznym
impl HomophonicData {
    /// Tworzy nową strukturę z informacjami szyfru homofonicznego
    /// 
    /// # Arguments
    /// 
    /// * `code` - klucz do wykorzystania (musi spełniać warunki[^1][^2])
    /// 
    /// # Errors
    /// 
    /// * [^1]Klucz musi posiadać co najmniej 52 znaki w linii bądź kolumnie
    /// * [^2]Klucz musi posiadać unikalne znaki
    pub fn new(code: Key) -> Result<HomophonicData, Box<dyn std::error::Error>> {
        if code.data.len() < 52 { 
            return Err("Invalid code! Valid code must contain at least 52 characters".into()) 
        } else if !code.unique {
            return Err("Invalid code! Valid code must contain unique characters".into())
        }
        Ok(HomophonicData(code))
    }
}

/// Implementacja metod szyfrujących
impl Cipher for HomophonicData {
    fn encrypt(&self, data: &str) -> String { 
        let mut rng: ThreadRng = rand::thread_rng();
        data.chars()
            .map(|sign: char| {
                if !sign.is_whitespace() && sign.is_ascii() { 
                    let char_key: &String;
                    if sign.is_ascii_lowercase() {
                        char_key = &self.0.data[((sign as usize - 'a' as usize) % 52 + 26)]
                    } else {
                        char_key = &self.0.data[((sign as usize - 'A' as usize) % 52)]
                    }
                    char_key.chars()
                     .nth(rng.gen_range(0..char_key.len()))
                     .unwrap()
                } else { 
                    sign 
                } 
            })
            .collect::<String>()
    }
    fn decrypt(&self, data: &str) -> String { 
        data.chars()
            .map(|sign: char| {
                if !sign.is_whitespace() && sign.is_ascii() { 
                    let index = self.0.data.iter()
                                               .position(|v: &String| v.contains(sign))
                                               .unwrap() as u8;
                    if index < 26 {
                        ('A' as u8 + index) as char
                    } else {
                        ('a' as u8 + index % 26) as char
                    }
                } else { 
                    sign 
                } 
            })
            .collect::<String>()
    }
}
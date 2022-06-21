use crate::algorithms::Cipher;

/// Przechowuje dodatkowe infromacje potrzebne dla szyfru cezara
pub struct CeasarData(u8);

/// Metody dodatkowe związane z szyfrem cezara
impl CeasarData {
    /// Tworzy nową strukturę z informacjami szyfru cezara
    /// 
    /// # Arguments
    /// 
    /// * `offset` - przesunięcie znaków
    pub fn new(offset: u8) -> CeasarData {
        CeasarData(offset)
    }

    /// Przesuwa pojedynczy znak o podaną liczbę miejsc w alfabecie łacińskim
    /// 
    /// # Arguments
    /// 
    /// * `character` - znak do przesunięcia
    /// * `offset` - wartość przesunięcia
    pub fn shift_ascii(character: char, offset: i8) -> char {
        let mut shift: i8 = 'A' as i8;
        if character.is_ascii_lowercase() {
            shift = 'a' as i8;
        } 
        ((((character as i8) + offset - shift) % 26) + shift) as u8 as char
    }
}

/// Implementacja metod szyfrujących
impl Cipher for CeasarData {
    fn encrypt(&self, data: &str) -> String{
        data.chars()
            .map(|sign: char| {
                if sign.is_ascii_alphabetic() { 
                    CeasarData::shift_ascii(sign, self.0 as i8)
                }
                else { 
                    sign 
                } 
            })
            .collect::<String>()
    }
    fn decrypt(&self, data: &str) -> String{
        data.chars()
            .map(|sign| {
                if sign.is_ascii_alphabetic() { 
                    CeasarData::shift_ascii(sign, -(self.0 as i8))
                }
                else { 
                    sign 
                } 
            })
            .collect::<String>()
    }
}
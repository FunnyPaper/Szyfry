pub mod ceasar;
pub mod homophonics;
pub mod polyalphabetic;

pub use ceasar::CeasarData;
pub use homophonics::HomophonicData;
pub use polyalphabetic::PolyalphabeticData;

/// Wspólny interfejs szyfrów
pub trait Cipher {
    /// Metoda do szyfrowania podanego ciągu znaków
    /// 
    /// # Arguments
    /// 
    /// * `data` - ciąg znaków do zaszyfrowania
    fn encrypt(&self, data: &str) -> String;

    /// Metoda do odszyfrowywania podanego ciągu znaków
    /// 
    /// # Arguments
    /// 
    /// * `data` - ciąg znaków do odszyfrowania
    fn decrypt(&self, data: &str) -> String;
}
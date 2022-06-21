use std::{fs, error::Error};
use rand::{Rng, prelude::{SliceRandom, ThreadRng}};
use core::cmp;

/// Do określania w jaki sposób odczytywać pliki
pub enum KeyRule {
    /// Linijka po linijce
    Row,
    /// Kolumna po kolumnie
    Column,
}

/// Do przechowywania klucza szyfrującego
pub struct Key
{
    /// Reprzentacja klucza jako wektor (w zależności od sposobu odczytywania)
    pub data: Vec<String>,
    /// Sposób reprezentacji klucza
    pub rule: KeyRule,
    /// Czy klucz zawiera unikalne znaki
    pub unique: bool
}

/// Metody klucza
impl Key {
    /// Zczytuje dane z pliku i na ich podstawie tworzy instancje klucza
    /// 
    /// # Arguments
    /// 
    /// * `path` - ścieżka odczytu
    /// * `rule` - sposób odczytu klucza
    /// 
    /// # Errors
    /// 
    /// Błędy związane z odczytem pliku (np. nieistniejąca ścieżka)
    pub fn read(path: &str, rule: KeyRule) -> Result<Key, Box<dyn Error>> {
        let str: String = fs::read_to_string(path)?;

        // zczytaj dane w zależności od podanej zasady
        let mut data: Vec<String> = Vec::new();
        match rule {
            KeyRule::Row => {
                data.resize(str.lines().count(), String::new());
                str .lines()
                    .zip(&mut data)
                    .for_each(|(l, v): (&str, &mut String)| 
                        v.push_str(&l.split_whitespace()
                                             .collect::<String>())
                    );
            },
            KeyRule::Column => {
                let cap: usize = str.lines()
                                    .max_by_key(|&l: &&str| l.split_whitespace()
                                                            .collect::<String>()
                                                            .len())
                                    .unwrap()
                                    .len();
                data.resize(cap, String::new());
                str .lines()
                    .for_each(|l: &str| l.chars()
                                         .zip(&mut data)
                                         .filter(|(l, _d): &(char, &mut String)| !l.is_whitespace())
                                         .for_each(|(c, s): (char, &mut String)| s.push(c)));
            }
        }
        let unique: bool = !(1..data.len()).any(|i| data[i..].contains(&data[i - 1]));        
        Ok(Key { data, rule, unique })
    }

    /// Zczytuje dane z pliku (katalog nadrzędny to translation_keys) i na ich podstawie tworzy instancje klucza
    /// 
    /// # Arguments
    /// 
    /// * `path` - ścieżka odczytu
    /// * `rule` - sposób odczytu klucza
    /// 
    /// # Errors
    /// 
    /// Błędy związane z odczytem pliku (np. nieistniejąca ścieżka)
    pub fn read_translation(path: &str, rule: KeyRule) -> Result<Key, Box<dyn Error>> {
        let path: String = format!("translation_keys/{}", path);
        Key::read(&path, rule)
    }

    /// Zapisuje klucz do pliku
    /// 
    /// # Arguments
    /// 
    /// * `path` - ścieżka zapisu
    pub fn write(&self, path: &str) {
        let output: String = self.to_string();
        fs::write(path, output).expect("Writing error!");
    } 

    /// Zapisuje klucz do pliku (katalog nadrzędny to translation_keys)
    /// 
    /// # Arguments
    /// 
    /// * `path` - ścieżka zapisu
    pub fn write_translation(&self, path: &str) {
        let path: String = format!("translation_keys/{}", path);
        self.write(&path); 
    }

    /// Tworzy klucz na podstawie paramterów
    /// 
    /// # Arguments
    /// 
    /// * `rule` - sposób odczytu klucza
    /// * `char_count` - ilość różnych znaków do generacji (maksymalna długość linijki/ kolumny) 
    /// * `max_sub_count` - ilość różnych znaków do generacji na daną pozycję
    /// * `char_range` - znaki brane pod uwagę przy generacji
    /// * `unique` - czy klucz ma zawierać tylko unikalne znaki
    pub fn generate(rule: KeyRule, char_count: usize, max_sub_count: usize, char_range: Vec<char>, unique: bool) -> Key {
        match unique {
            true => Key::unique_generate(rule, char_count, max_sub_count, char_range),
            false => Key::non_unique_generate(rule, char_count, max_sub_count, char_range)
        }
    }

    fn unique_generate(rule: KeyRule, char_count: usize, max_sub_count: usize, mut char_range: Vec<char>) -> Key {
        let mut rng: ThreadRng = rand::thread_rng();
        char_range.shuffle(&mut rng);

        let mut data: Vec<String> = char_range.drain(..cmp::min(char_count, char_range.len()))
                                              .map(|c: char| c.to_string())
                                              .collect();

        for keys in &mut data {
            let amount: usize = rng.gen_range(0..=cmp::min(char_range.len(), max_sub_count - 1));
            if char_range.len() >= amount {
                keys.extend(char_range.drain(..amount));
            }
        }
        Key { data, rule, unique: true }
    }

    fn non_unique_generate(rule: KeyRule, char_count: usize, max_sub_count: usize, mut char_range: Vec<char>) -> Key {
        let mut rng: ThreadRng = rand::thread_rng();
        char_range.shuffle(&mut rng);

        let mut data: Vec<String> = Vec::new();
        data.resize(cmp::min(char_range.len(), char_count), String::new());

        for keys in &mut data {
            let amount: usize = rng.gen_range(1..=cmp::min(char_range.len(), max_sub_count));
            if char_range.len() >= amount {
                keys.extend(char_range.choose_multiple(&mut rng, amount))
            }
        }
        Key { data, rule, unique: false }
    }
}

/// Konwersja klucza na ciąg znaków
impl ToString for Key {
    fn to_string(&self) -> String {
        match self.rule {
            KeyRule::Row => self.data.join("\n"),
            KeyRule::Column => {
                let max: usize = self.data.iter()
                                          .max_by_key(|&v: &&String| v.len())
                                          .unwrap()
                                          .len();
                let mut output: String = String::new();
                for i in 0usize..max {
                    for v in &self.data {
                        if let Some(ch) = v.chars().nth(i) {
                            output.push(ch);
                        } else {
                            output.push(' ');
                        }
                    }
                    output.push('\n');
                }
                output.pop();
                output
            }
        }
    }
}
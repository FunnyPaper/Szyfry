use std::io::Stdin;
use std::{io, error::Error};
use szyfry::{algorithms::*, utils::*};

fn main() {
    println!("Welcome to cipher program!\n");
    while prompt_program() {
        println!("Welcome back!\n")
    } 
    println!("See you soon!");
}

/// Do określania czy należy zaszyfrować czy odszyfrować tekst
#[derive(Debug)]
enum OperationName {
    /// Szyfrowanie
    Encryption, 
    /// Odszyfrowywanie
    Decryption
}

/// Do określenia jaki algorytm wykorzystać
#[derive(Debug)]
enum CipherName {
    /// Szyfr cezara
    Ceasar,
    /// Szyfr homofoniczny
    Homophonic,
    /// Szyfr wieloalfabetowy (Vigenère'a)
    Polyalphabetic
}

/// Uruchamia serię pytań konsolowych
fn prompt_program() -> bool {
    let choice: OperationName = loop_error(prompt_choice);
    let cipher: CipherName = loop_error(prompt_cipher);
    let algorithm: Box<dyn Cipher> = match cipher {
        CipherName::Ceasar => loop_error(prompt_ceasar_data),
        CipherName::Homophonic => loop_error(prompt_homophonic_data),
        CipherName::Polyalphabetic => loop_error(prompt_polyalphabetic_data)
    };
    let plain_text: String = loop_error(prompt_data);
    let cipher_text: String = match choice {
        OperationName::Encryption => algorithm.encrypt(&plain_text),
        OperationName::Decryption => algorithm.decrypt(&plain_text)
    };
    println!("After {:#?} with {:#?}\n{}\ngives:\n{}", choice, cipher, plain_text, cipher_text);
    loop_error(prompt_repeat_program)
}

/// Pobiera od użytkownika informację o zakończeniu programu
fn prompt_repeat_program() -> Result<bool, Box<dyn Error>> {
    println!("Would you like to use this program once again?[Y/n]:");

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;
    match buffer.trim().to_lowercase().as_str() {
        "y" => Ok(true),
        "n" => Ok(false),
        _ => Err(format!("INVALID OPTION: {}\nVALID OPTIONS ARE: \n\ty\n\tn", buffer).into())
    }
}

/// Pobiera od użytkownika informację o tym czy zaszyfrować/ odszyfrować ciąg znaków
fn prompt_choice() -> Result<OperationName, Box<dyn Error>> {
    println!("Choose action\n1. encrypt\n2. decrypt");
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;
    match buffer.trim().to_lowercase().as_str() {
        "1" | "encrypt" | "e" => Ok(OperationName::Encryption),
        "2" | "decrypt" | "d" => Ok(OperationName::Decryption),
        _ => Err(format!("INVALID OPTION: {}\nVALID OPTIONS ARE: \n\t1, encrypt, e\n\t2, decrypt, d", buffer).into())
    } 
}

/// Pobiera od użytkownika informację o algorytmie do wykorzystania
fn prompt_cipher() -> Result<CipherName, Box<dyn Error>> {
    println!("Choose algorithm\n1. ceasar\n2. homophonic\n3. polyalphabetic");
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;
    match buffer.trim().to_lowercase().as_str() {
        "1" | "ceasar" | "c" => Ok(CipherName::Ceasar),
        "2" | "homophonic" | "h" => Ok(CipherName::Homophonic),
        "3" | "polyalphabetic" | "p" => Ok(CipherName::Polyalphabetic),
        _ => Err(format!("INVALID OPTION: {}\nVALID OPTIONS ARE: \n\t1, ceasar, c\n\t2, homophonic, h\n\t3, polyalphabetic, p", buffer).into())
    } 
}

/// Pobiera od użytkownika informację o kluczu
fn prompt_keycode() -> Result<Key, Box<dyn Error>> {
    let stdin: Stdin = io::stdin();
    let mut data: Vec<String> = vec![String::new(); 2];
    println!("Pass full path to key:");
    stdin.read_line(&mut data[0])?;
    println!("How to handle your key?\n1. column\n2. row");
    stdin.read_line(&mut data[1])?;
    let rule: KeyRule = match data[1].trim() {
        "1" | "column" | "c" => KeyRule::Column,
        "2" | "row" | "r" => KeyRule::Row,
        _ => return Err(format!("INVALID OPTION: {}\nVALID OPTIONS ARE: \n\t1, column, c\n\t2, row, r", data[1]).into())
    };
    Key::read(data[0].trim(), rule)
}

/// Pobiera od użytkownika informację o przesunięciu (szyfr cezara)
fn prompt_ceasar_data() -> Result<Box<dyn Cipher>, Box<dyn Error>> {
    println!("Pass offset:");
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;
    let offset: u8 = buffer.trim().parse()?;
    Ok(Box::new(CeasarData::new(offset))) 
}

/// Pobiera od użytkownika informację o kluczu (szyfr homofoniczny)
fn prompt_homophonic_data() -> Result<Box<dyn Cipher>, Box<dyn Error>> {
    let code: Key = prompt_keycode()?;
    let algorithm: HomophonicData = HomophonicData::new(code)?;
    Ok(Box::new(algorithm)) 
}

/// Pobiera od użytkownika informację o kluczu (szyfr wieloalfabetowy)
fn prompt_polyalphabetic_data() -> Result<Box<dyn Cipher>, Box<dyn Error>> {
    let code: Key = prompt_keycode()?;
    let algorithm: PolyalphabeticData = PolyalphabeticData::new(code)?;
    Ok(Box::new(algorithm))
}

/// Pobiera od użytkownika informację o ciągu znaków
fn prompt_data() -> Result<String, Box<dyn Error>> {
    println!("Pass text to cipher:");
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

/// Powtarza podaną funkcję w nieskończoność w przypadku kiedy zwróci ona błąd
fn loop_error<T>(repeat: fn() -> Result<T, Box<dyn Error>>) -> T {
    let mut output: Result<T, Box<dyn Error>> = repeat();
    while let Err(error) = output {
        eprintln!("{}", error);
        output = repeat();
    };
    output.unwrap()
}
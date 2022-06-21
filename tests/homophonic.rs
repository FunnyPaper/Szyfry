use szyfry::algorithms::{Cipher, HomophonicData};
use szyfry::utils::{Key, KeyRule};

#[test]
fn encrypt_decrypt_ckey() {
    let data: &str = "Ala ma kota";
    let code: Key = Key::read_translation("test.ckey", KeyRule::Column).unwrap();
    let algorithm: HomophonicData = HomophonicData::new(code).unwrap();

    let encrypted: String = algorithm.encrypt(data);
    let decrypted: String = algorithm.decrypt(&encrypted);

    assert_eq!(decrypted, data);
}

#[test]
fn encrypt_decrypt_lkey() {
    let data: &str = "Ala ma kota";
    let code: Key = Key::read_translation("test.lkey", KeyRule::Row).unwrap();
    let algorithm: HomophonicData = HomophonicData::new(code).unwrap();

    let encrypted: String = algorithm.encrypt(data);
    let decrypted: String = algorithm.decrypt(&encrypted);

    assert_eq!(decrypted, data);
}
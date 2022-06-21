use szyfry::algorithms::{Cipher, PolyalphabeticData};
use szyfry::utils::{Key, KeyRule};

#[test]
fn encrypt_decrypt_ckey() {
    let data: &str = "Ala ma kota";
    let code: Key = Key::read_translation("ascii_only.ckey", KeyRule::Column).unwrap();
    let algorithm: PolyalphabeticData = PolyalphabeticData::new(code).unwrap();

    let encrypted: String = algorithm.encrypt(data);
    let decrypted: String = algorithm.decrypt(&encrypted);

    assert_eq!(decrypted, data);
}

#[test]
fn encrypt_decrypt_lkey() {
    let data: &str = "Ala ma kota";
    let code: Key = Key::read_translation("ascii_only.lkey", KeyRule::Row).unwrap();
    let algorithm: PolyalphabeticData = PolyalphabeticData::new(code).unwrap();

    let encrypted: String = algorithm.encrypt(data);
    let decrypted: String = algorithm.decrypt(&encrypted);

    assert_eq!(decrypted, data);
}
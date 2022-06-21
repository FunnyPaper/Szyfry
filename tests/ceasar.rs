use szyfry::algorithms::Cipher;
use szyfry::algorithms::ceasar::CeasarData;

#[test]
fn encryption_by1() {
    let data: &str = "Ala ma kota";
    let offset: u8 = 1;
    let ceasar: Box<dyn Cipher> = Box::new(CeasarData::new(offset));

    let output: String = ceasar.encrypt(data);

    assert_eq!(output, "Bmb nb lpub");
}

#[test]
fn encryption_by2() {
    let data: &str = "Z z , y";
    let offset: u8 = 2;
    let ceasar: Box<dyn Cipher> = Box::new(CeasarData::new(offset));

    let output: String = ceasar.encrypt(data);

    assert_eq!(output, "B b , a");
}

#[test]
fn decrytpion_by2() {
    let data: &str = "cde";
    let offset: u8 = 2;
    let ceasar: Box<dyn Cipher> = Box::new(CeasarData::new(offset));

    let output: String = ceasar.decrypt(data);

    assert_eq!(output, "abc");
}
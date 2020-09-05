use rasn::{ber, cer, der, types};

#[test]
fn arith1() {
    if let Ok(value) = ber::decode::<types::Open>(&*include_bytes!("data/arith1.bin")) {
        assert_eq!(value, ber::decode(&ber::encode(&value).unwrap()).unwrap());
    }
}

#[test]
fn arith2() {
    if let Ok(value) = ber::decode::<types::Open>(&*include_bytes!("data/arith2.bin")) {
        let _ = ber::encode(&value);
    }
}

#[test]
fn int161() {
    if let Ok(value) = ber::decode::<types::Open>(&*include_bytes!("data/int161.bin")) {
        let encode = &ber::encode(&value).unwrap();
        match value {
            types::Open::BitString(_) => {}
            _ => assert_eq!(value, ber::decode(&encode).unwrap()),
        }
    }
}

#[test]
fn int162() {
    let bytes = &*include_bytes!("data/int162.bin");
    println!("{:?}", bytes);
    if let Ok(value) = ber::decode::<types::Open>(bytes) {
        println!("{:?}", value);
        let bytes = &ber::encode(&value).unwrap();
        println!("{:?}", bytes);
        assert_eq!(value, ber::decode(&bytes).unwrap());
    }
}

#[test]
fn int321() {
    if let Ok(value) = ber::decode::<types::Open>(&*include_bytes!("data/int321.bin")) {
        assert_eq!(value, ber::decode(&ber::encode(&value).unwrap()).unwrap());
    }
}

#[test]
fn int322() {
    if let Ok(value) = ber::decode::<types::Open>(&*include_bytes!("data/int322.bin")) {
        assert_eq!(value, ber::decode(&ber::encode(&value).unwrap()).unwrap());
    }
}

#[test]
fn int323() {
    if let Ok(value) = ber::decode::<types::Open>(&*include_bytes!("data/int323.bin")) {
        assert_eq!(value, ber::decode(&ber::encode(&value).unwrap()).unwrap());
    }
}

#[test]
fn int324() {
    let bytes = &*include_bytes!("data/int324.bin");
    println!("{:?}", bytes);
    if let Ok(value) = ber::decode::<types::Open>(bytes) {
        println!("{:?}", value);
        let bytes = &ber::encode(&value).unwrap();
        println!("{:?}", bytes);
        assert_eq!(value, ber::decode(&bytes).unwrap());
    }
}

#[test]
fn int325() {
    let data = &*include_bytes!("data/int325.bin");
    println!("{:?}", data);

    if let Ok(value) = ber::decode::<types::Open>(data) {
        println!("{:?}", value);
        let bytes = &ber::encode(&value).unwrap();
        println!("{:?}", bytes);
        assert_eq!(value, ber::decode(&bytes).unwrap());
    }

    if let Ok(value) = cer::decode::<types::Open>(data) {
        println!("{:?}", value);
        assert_eq!(value, cer::decode(&cer::encode(&value).unwrap()).unwrap());
    }

    if let Ok(value) = der::decode::<types::Open>(data) {
        println!("{:?}", value);
        assert_eq!(value, der::decode(&der::encode(&value).unwrap()).unwrap());
    }
}

#[test]
fn havoc1() {
    let _ = rasn::ber::decode::<bool>(&*include_bytes!("data/havoc1.bin"));
}

#[test]
fn havoc2() {
    let _ = rasn::ber::decode::<types::Open>(&*include_bytes!("data/havoc2.bin"));
}

#[test]
fn havoc3() {
    let bytes = &*include_bytes!("data/havoc3.bin");
    println!("{:?}", bytes);
    if let Ok(value) = ber::decode::<types::Open>(bytes) {
        println!("{:?}", value);
        let bytes = &ber::encode(&value).unwrap();
        println!("{:?}", bytes);
        assert_eq!(value, ber::decode(&bytes).unwrap());
    }
}

#[test]
fn havoc4() {
    let bytes = &*include_bytes!("data/havoc4.bin");
    println!("{:?}", bytes);
    if let Ok(value) = ber::decode::<types::Open>(bytes) {
        println!("{:?}", value);
        let bytes = &ber::encode(&value).unwrap();
        println!("{:?}", bytes);
        assert_eq!(value, ber::decode(bytes).unwrap());
    }
}

#[test]
fn havoc5() {
    let bytes = &*include_bytes!("data/havoc5.bin");
    println!("{:?}", bytes);
    if let Ok(value) = ber::decode::<types::Open>(bytes) {
        println!("{:?}", value);
        let bytes = &ber::encode(&value).unwrap();
        println!("{:?}", bytes);
        assert_eq!(value, ber::decode(&bytes).unwrap());
    }
}

#[test]
fn havoc6() {
    if let Ok(value) = ber::decode::<types::Open>(&*include_bytes!("data/havoc6.bin")) {
        let _ = ber::encode(&value);
    }
}

#[test]
fn flip1() {
    let _ = rasn::ber::decode::<types::Open>(&*include_bytes!("data/flip1.bin"));
}

#[test]
fn flip2() {
    assert!(ber::decode::<types::Open>(&*include_bytes!("data/flip2.bin")).is_err());
}

#[test]
fn flip3() {
    if let Ok(value) = ber::decode::<types::Open>(&*include_bytes!("data/flip3.bin")) {
        let _ = ber::encode(&value);
    }
}

#[test]
fn flip4() {
    let data = &*include_bytes!("data/flip4.bin");

    println!("{:?}", data);
    if let Ok(value) = ber::decode::<types::Open>(data) {
        println!("{:?}", value);
        let encoded = &ber::encode(&value).unwrap();
        println!("{:?}", encoded);
        assert_eq!(value, ber::decode(&encoded).unwrap());
    }

    if let Ok(value) = cer::decode::<types::Open>(data) {
        assert_eq!(value, cer::decode(&cer::encode(&value).unwrap()).unwrap());
    }

    if let Ok(value) = der::decode::<types::Open>(data) {
        assert_eq!(value, der::decode(&der::encode(&value).unwrap()).unwrap());
    }
}

#[test]
fn havoc7() {
    let data = &*include_bytes!("data/havoc7.bin");

    println!("{:?}", data);
    if let Ok(value) = ber::decode::<types::Open>(data) {
        println!("{:?}", value);
        let encoded = &ber::encode(&value).unwrap();
        println!("{:?}", encoded);
        assert_eq!(value, ber::decode(&encoded).unwrap());
    }

    if let Ok(value) = cer::decode::<types::Open>(data) {
        assert_eq!(value, cer::decode(&cer::encode(&value).unwrap()).unwrap());
    }

    if let Ok(value) = der::decode::<types::Open>(data) {
        assert_eq!(value, der::decode(&der::encode(&value).unwrap()).unwrap());
    }
}

#[test]
fn havoc8() {
    let data = &*include_bytes!("data/havoc8.bin");

    println!("{:?}", data);
    if let Ok(value) = ber::decode::<types::Open>(data) {
        println!("{:?}", value);
        let encoded = &ber::encode(&value).unwrap();
        println!("{:?}", encoded);
        assert_eq!(value, ber::decode(&encoded).unwrap());
    }

    if let Ok(value) = cer::decode::<types::Open>(data) {
        println!("{:?}", value);
        let encoded = &cer::encode(&value).unwrap();
        println!("{:?}", encoded);
        assert_eq!(value, cer::decode(&encoded).unwrap());
    }

    if let Ok(value) = der::decode::<types::Open>(data) {
        assert_eq!(value, der::decode(&der::encode(&value).unwrap()).unwrap());
    }
}

#[test]
fn havoc9() {
    let data = &*include_bytes!("data/havoc9.bin");

    println!("{:?}", data);
    if let Ok(value) = ber::decode::<types::Open>(data) {
        println!("{:?}", value);
        let encoded = &ber::encode(&value).unwrap();
        println!("{:?}", encoded);
        assert_eq!(value, ber::decode(&encoded).unwrap());
    }

    if let Ok(value) = cer::decode::<types::Open>(data) {
        println!("{:?}", value);
        let encoded = &cer::encode(&value).unwrap();
        println!("{:?}", encoded);
        assert_eq!(value, cer::decode(&encoded).unwrap());
    }

    if let Ok(value) = der::decode::<types::Open>(data) {
        assert_eq!(value, der::decode(&der::encode(&value).unwrap()).unwrap());
    }
}

#[test]
fn flip5() {
    let data = &*include_bytes!("data/flip5.bin");

    println!("{:?}", data);
    if let Ok(value) = ber::decode::<types::Open>(data) {
        println!("{:?}", value);
        let encoded = &ber::encode(&value).unwrap();
        println!("{:?}", encoded);
        assert_eq!(value, ber::decode(&encoded).unwrap());
    }

    if let Ok(value) = cer::decode::<types::Open>(data) {
        println!("{:?}", value);
        let encoded = &cer::encode(&value).unwrap();
        println!("{:?}", encoded);
        assert_eq!(value, cer::decode(&encoded).unwrap());
    }

    if let Ok(value) = der::decode::<types::Open>(data) {
        assert_eq!(value, der::decode(&der::encode(&value).unwrap()).unwrap());
    }
}

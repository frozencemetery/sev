use super::*;

#[test]
fn decode() {
    sev::Certificate::decode(&mut &CEK[..], ()).unwrap();
}

#[test]
fn encode() {
    let cek = sev::Certificate::decode(&mut &CEK[..], ()).unwrap();

    let mut output = Vec::new();
    cek.encode(&mut output, ()).unwrap();
    assert_eq!(CEK.len(), output.len());
    assert_eq!(CEK.to_vec(), output);
}

#[cfg(feature = "openssl")]
#[test]
fn verify() {
    let ask = ca::Certificate::decode(&mut ASK, ()).unwrap();
    let cek = sev::Certificate::decode(&mut CEK, ()).unwrap();

    (&ask, &cek).verify().unwrap();
    //assert!((&cek, &ask).verify().is_err());
}

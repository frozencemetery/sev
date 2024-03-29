use sev::{Build, Version, firmware::Firmware, certs::sev::Usage};

#[ignore]
#[test]
fn platform_reset() {
    let fw = Firmware::open().unwrap();
    fw.platform_reset().unwrap();
}

#[cfg_attr(not(has_sev), ignore)]
#[test]
fn platform_status() {
    let fw = Firmware::open().unwrap();
    let status = fw.platform_status().unwrap();
    assert!(status.build > Build(Version(0, 14), 0));
}

#[ignore]
#[test]
fn pek_generate() {
    let fw = Firmware::open().unwrap();
    fw.pek_generate().unwrap();
}

#[cfg_attr(not(has_sev), ignore)]
#[test]
fn pek_csr() {
    let fw = Firmware::open().unwrap();
    let pek = fw.pek_csr().unwrap();
    assert_eq!(pek, Usage::PEK);
}

#[ignore]
#[test]
fn pdh_generate() {
    let fw = Firmware::open().unwrap();
    fw.pdh_generate().unwrap();
}

#[cfg_attr(not(has_sev), ignore)]
#[cfg(feature = "openssl")]
#[test]
fn pdh_cert_export() {
    use sev::certs::Verifiable;

    let fw = Firmware::open().unwrap();
    let chain = fw.pdh_cert_export().unwrap();

    assert_eq!(chain.pdh, Usage::PDH);
    assert_eq!(chain.pek, Usage::PEK);
    assert_eq!(chain.oca, Usage::OCA);
    assert_eq!(chain.cek, Usage::CEK);

    chain.verify().unwrap();
}

#[cfg(feature = "openssl")]
#[ignore]
#[test]
fn pek_cert_import() {
    use sev::certs::{Signer, Verifiable, sev::Certificate};

    let fw = Firmware::open().unwrap();

    let (mut oca, key) = Certificate::generate(Usage::OCA).unwrap();
    key.sign(&mut oca).unwrap();

    let mut pek = fw.pek_csr().unwrap();
    key.sign(&mut pek).unwrap();

    fw.pek_cert_import(&pek, &oca).unwrap();

    let chain = fw.pdh_cert_export().unwrap();
    assert_eq!(oca, chain.oca);
    chain.verify().unwrap();

    fw.platform_reset().unwrap();
}

#[cfg_attr(not(has_sev), ignore)]
#[test]
fn get_identifer() {
    let fw = Firmware::open().unwrap();
    let id = fw.get_identifer().unwrap();
    assert_ne!(Vec::from(id), vec![0u8; 64]);
}

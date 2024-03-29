#![cfg(feature = "openssl")]

mod initialized {
    use ::sev::{launch, session::Session, certs::*};
    use codicon::Decoder;
    use std::convert::*;

    #[test]
    fn create() {
        Session::try_from(launch::Policy::default()).unwrap();
    }

    #[test]
    fn start() {
        const ARK: &[u8] = include_bytes!("naples/ark.cert");
        const ASK: &[u8] = include_bytes!("naples/ask.cert");
        const CEK: &[u8] = include_bytes!("naples/cek.cert");
        const OCA: &[u8] = include_bytes!("naples/oca.cert");
        const PEK: &[u8] = include_bytes!("naples/pek.cert");
        const PDH: &[u8] = include_bytes!("naples/pdh.cert");

        let session = Session::try_from(launch::Policy::default()).unwrap();
        session.start(Chain {
            ca: ca::Chain {
                ark: ca::Certificate::decode(&mut &ARK[..], ()).unwrap(),
                ask: ca::Certificate::decode(&mut &ASK[..], ()).unwrap(),
            },
            sev: sev::Chain {
                cek: sev::Certificate::decode(&mut &CEK[..], ()).unwrap(),
                oca: sev::Certificate::decode(&mut &OCA[..], ()).unwrap(),
                pek: sev::Certificate::decode(&mut &PEK[..], ()).unwrap(),
                pdh: sev::Certificate::decode(&mut &PDH[..], ()).unwrap(),
            },
        }).unwrap();
    }
}

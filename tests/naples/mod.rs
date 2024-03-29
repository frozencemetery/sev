mod ark;
mod ask;
mod cek;
mod oca;
mod pek;
mod pdh;

const ARK_BAD: &[u8] = include_bytes!("ark.cert.bad");

const OCA: &[u8] = include_bytes!("oca.cert");
const ARK: &[u8] = include_bytes!("ark.cert");
const ASK: &[u8] = include_bytes!("ask.cert");
const CEK: &[u8] = include_bytes!("cek.cert");
const PEK: &[u8] = include_bytes!("pek.cert");
const PDH: &[u8] = include_bytes!("pdh.cert");

use ::sev::certs::*;

#[allow(unused_imports)]
use codicon::*;

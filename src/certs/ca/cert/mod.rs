mod v1;

use super::*;

#[repr(C)]
pub union Certificate {
    version: u32,
    v1: v1::Certificate,
}

impl std::fmt::Debug for Certificate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.version() {
            1 => write!(f, "{:?}", unsafe { self.v1 }),
            v => write!(f, "Certificate {{ version: {} }}", v),
        }
    }
}

#[cfg(feature = "openssl")]
impl std::fmt::Display for Certificate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use codicon::Encoder;
        use std::fmt::Error;

        let key = PublicKey::try_from(self).or(Err(Error))?;

        let mut hsh = hash::Hasher::new(key.hash)?;

        self.encode(&mut hsh, Body).or(Err(Error))?;

        write!(f, "{} {} ", crate::certs::Usage::from(key.usage), key)?;
        for b in hsh.finish()?.iter() {
            write!(f, "{:02x}", *b)?;
        }

        Ok(())
    }
}

impl Eq for Certificate {}
impl PartialEq for Certificate {
    fn eq(&self, other: &Certificate) -> bool {
        if unsafe { self.version != other.version } { return false; }
        match self.version() {
            1 => unsafe { self.v1 == other.v1 },
            _ => false,
        }
    }
}

impl<U: Copy + Into<crate::certs::Usage>> PartialEq<U> for Certificate {
    fn eq(&self, other: &U) -> bool {
        if let Ok(a) = Usage::try_from(self) {
            return a == (*other).into()
        }

        false
    }
}

impl codicon::Decoder for Certificate {
    type Error = Error;

    fn decode(reader: &mut impl Read, params: ()) -> Result<Self> {
        Ok(match u32::from_le(reader.load()?) {
            1 => Certificate { v1: v1::Certificate::decode(reader, params)? },
            _ => return Err(ErrorKind::InvalidData.into()),
        })
    }
}

impl codicon::Encoder for Certificate {
    type Error = Error;

    fn encode(&self, writer: &mut impl Write, _: ()) -> Result<()> {
        match self.version() {
            1 => unsafe { self.v1.encode(writer, ()) },
            _ => Err(ErrorKind::InvalidInput.into()),
        }
    }
}

#[cfg(feature = "openssl")]
impl codicon::Encoder<Body> for Certificate {
    type Error = Error;

    fn encode(&self, writer: &mut impl Write, _: Body) -> Result<()> {
        match self.version() {
            1 => unsafe { self.v1.encode(writer, Body) },
            _ => Err(ErrorKind::InvalidInput.into()),
        }
    }
}

impl TryFrom<&Certificate> for Usage {
    type Error = Error;

    fn try_from(value: &Certificate) -> Result<Self> {
        match value.version() {
            1 => Ok(unsafe { value.v1.preamble.data.usage }),
            _ => Err(ErrorKind::InvalidInput.into()),
        }
    }
}

impl TryFrom<&Certificate> for crate::certs::Usage {
    type Error = Error;

    fn try_from(value: &Certificate) -> Result<Self> {
        Ok(Usage::try_from(value)?.into())
    }
}

#[cfg(feature = "openssl")]
impl TryFrom<&Certificate> for PublicKey<Usage> {
    type Error = Error;

    fn try_from(value: &Certificate) -> Result<Self> {
        match value.version() {
            1 => unsafe { value.v1.try_into() },
            _ => Err(ErrorKind::InvalidInput.into()),
        }
    }
}

#[cfg(feature = "openssl")]
impl TryFrom<&Certificate> for Signature {
    type Error = Error;

    #[inline]
    fn try_from(value: &Certificate) -> Result<Self> {
        match value.version() {
            1 => unsafe { Ok(value.v1.try_into()?) },
            _ => Err(ErrorKind::InvalidInput.into()),
        }
    }
}

#[cfg(feature = "openssl")]
impl Verifiable for (&Certificate, &Certificate) {
    type Output = ();

    fn verify(self) -> Result<()> {
        let key: PublicKey<Usage> = self.0.try_into()?;
        let sig: Signature = self.1.try_into()?;
        key.verify(self.1, &sig)
    }
}

impl Certificate {
    #[inline]
    fn version(&self) -> u32 {
        u32::from_le(unsafe { self.version })
    }
}

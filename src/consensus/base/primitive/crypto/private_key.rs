use ed25519_dalek;
use rand::OsRng;
use beserial::{Serialize, Deserialize, ReadBytesExt, WriteBytesExt};
use crate::consensus::base::primitive::hash::{Hash, SerializeContent};

use crate::consensus::base::primitive::crypto::{PublicKey};
use std::io;
use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;

pub struct PrivateKey(pub(in super) ed25519_dalek::SecretKey);

impl PrivateKey {
    pub const SIZE: usize = 32;

    pub fn generate() -> Self {
        let mut cspring: OsRng = OsRng::new().unwrap();
        return PrivateKey(ed25519_dalek::SecretKey::generate(&mut cspring));
    }

    #[inline]
    pub fn as_bytes<'a>(&'a self) -> &'a [u8; PrivateKey::SIZE] { self.0.as_bytes() }

    #[inline]
    pub (crate) fn as_dalek<'a>(&'a self) -> &'a ed25519_dalek::SecretKey { &self.0 }
}

impl<'a> From<&'a [u8; PrivateKey::SIZE]> for PrivateKey {
    fn from(bytes: &'a [u8; PublicKey::SIZE]) -> Self {
        return PrivateKey(ed25519_dalek::SecretKey::from_bytes(bytes).unwrap());
    }
}

impl From<[u8; PrivateKey::SIZE]> for PrivateKey {
    fn from(bytes: [u8; PrivateKey::SIZE]) -> Self {
        return PrivateKey::from(&bytes);
    }
}

impl Clone for PrivateKey {
    fn clone(&self) -> Self {
        let cloned_dalek = ed25519_dalek::SecretKey::from_bytes(self.0.as_bytes()).unwrap();
        return PrivateKey(cloned_dalek);
    }
}

impl Deserialize for PrivateKey {
    fn deserialize<R: ReadBytesExt>(reader: &mut R) -> io::Result<Self> {
        let mut buf = [0u8; PrivateKey::SIZE];
        reader.read_exact(&mut buf)?;
        return Ok(PrivateKey::from(&buf));
    }
}

impl Serialize for PrivateKey {
    fn serialize<W: WriteBytesExt>(&self, writer: &mut W) -> io::Result<usize> {
        writer.write(self.as_bytes())?;
        return Ok(self.serialized_size());
    }

    fn serialized_size(&self) -> usize {
        return PrivateKey::SIZE;
    }
}

impl SerializeContent for PrivateKey {
    fn serialize_content<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> { self.serialize(writer) }
}

impl Hash for PrivateKey { }

impl Debug for PrivateKey {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "PrivateKey")
    }
}

impl PartialEq for PrivateKey {
    fn eq(&self, other: &PrivateKey) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl Eq for PrivateKey {}

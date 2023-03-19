use wasm_bindgen::prelude::*;

use nimiq_keys::SecureGenerate;

use crate::address::Address;
use crate::private_key::PrivateKey;
use crate::public_key::PublicKey;
use crate::signature::Signature;
use crate::signature_proof::SignatureProof;
use crate::transaction::Transaction;

/// A keypair represents a private key and its respective public key.
/// It is used for signing data, usually transactions.
#[wasm_bindgen]
pub struct KeyPair {
    inner: nimiq_keys::KeyPair,
}

#[wasm_bindgen]
impl KeyPair {
    /// Generates a new keypair from secure randomness.
    pub fn generate() -> KeyPair {
        let key_pair = nimiq_keys::KeyPair::generate_default_csprng();
        KeyPair::from_native(key_pair)
    }

    /// Derives a keypair from an existing private key.
    pub fn derive(private_key: &PrivateKey) -> KeyPair {
        let key_pair = nimiq_keys::KeyPair::from(private_key.native_ref().clone());
        KeyPair::from_native(key_pair)
    }

    /// Signs arbitrary data, returns a signature object.
    pub fn sign(&self, data: &[u8]) -> Signature {
        Signature::from_native(self.inner.sign(data))
    }

    /// Signs a transaction and sets the signature proof on the transaction object.
    #[wasm_bindgen(js_name = signTransaction)]
    pub fn sign_transaction(&self, transaction: &mut Transaction) {
        let signature = self.sign(transaction.serialize_content().as_ref());
        let proof = SignatureProof::single_sig(&self.public_key(), &signature);
        transaction.set_proof(proof.serialize());
    }

    /// Gets the keypair's private key.
    #[wasm_bindgen(getter, js_name = privateKey)]
    pub fn private_key(&self) -> PrivateKey {
        PrivateKey::from_native(self.inner.private.clone())
    }

    /// Gets the keypair's public key.
    #[wasm_bindgen(getter, js_name = publicKey)]
    pub fn public_key(&self) -> PublicKey {
        PublicKey::from_native(self.inner.public)
    }

    /// Gets the keypair's address.
    #[wasm_bindgen(js_name = toAddress)]
    pub fn to_address(&self) -> Address {
        Address::from_native(nimiq_keys::Address::from(&self.inner))
    }
}

impl KeyPair {
    pub fn from_native(key_pair: nimiq_keys::KeyPair) -> KeyPair {
        KeyPair { inner: key_pair }
    }

    pub fn native_ref(&self) -> &nimiq_keys::KeyPair {
        &self.inner
    }
}
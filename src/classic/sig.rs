use aes_gcm::aead::rand_core;
use k256::{
    ecdsa::{SigningKey, Signature, signature::Signer, VerifyingKey},
};

use k256::{ecdsa::{signature::Verifier}};

use rand_core::OsRng;

pub fn sig_gen() -> (SigningKey, VerifyingKey) {
    let pk = SigningKey::random(&mut OsRng);
    let sk = VerifyingKey::from(&pk);

    (pk, sk)

}

pub fn sig_sign(sk: &SigningKey, m: &Vec<u8>) -> Signature {
    let signature: Signature = sk.sign(m);

    signature

}

pub fn sig_vry(pk: &VerifyingKey, m: &Vec<u8>, signature: &Signature) -> bool {
    pk.verify(m, signature).is_ok()
}
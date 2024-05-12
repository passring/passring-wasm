use crate::keys::{private::PrivateKeyWrapper, public::PublicKeyWrapper};
use passring::signature::{FullSignature, PartialSignature};
use passring::traits::Random;
use passring::Passring;
use passring::PublicKey;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Passring)]
pub struct PassringWrapper(Passring);

#[wasm_bindgen(js_class = Passring)]
impl PassringWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(voting_id: String, ring: Vec<PublicKeyWrapper>) -> PassringWrapper {
        let ring = ring.into_iter().map(|pk| pk.0).collect();
        let passring = Passring::new(uuid::Uuid::parse_str(&voting_id).unwrap(), ring);
        PassringWrapper(passring)
    }

    #[wasm_bindgen]
    pub fn issue(&self, choice: u16, private_key: PrivateKeyWrapper) -> JsValue {
        let full_signature = self.0.issue(choice, private_key.0.clone());
        serde_wasm_bindgen::to_value(&full_signature).unwrap()
    }

    #[wasm_bindgen]
    pub fn validate(&self, full_signature: JsValue) -> bool {
        let full_signature: FullSignature = serde_wasm_bindgen::from_value(full_signature).unwrap();
        self.0.validate(full_signature)
    }

    #[wasm_bindgen]
    pub fn link(&self, full_signature_1: JsValue, full_signature_2: JsValue) -> bool {
        let full_signature_1: FullSignature =
            serde_wasm_bindgen::from_value(full_signature_1).unwrap();
        let full_signature_2: FullSignature =
            serde_wasm_bindgen::from_value(full_signature_2).unwrap();
        self.0.link(full_signature_1, full_signature_2)
    }
}

#[wasm_bindgen]
pub fn get_random_ring(size: usize) -> Vec<PublicKeyWrapper> {
    let ring: Vec<PublicKey> = (0..size)
        .map(|_| PublicKey::random(&mut rand::thread_rng()))
        .collect();
    ring.into_iter().map(|pk| PublicKeyWrapper(pk)).collect()
}

#[wasm_bindgen]
pub fn strip_full_signature(full_signature: JsValue) -> JsValue {
    let full_signature: FullSignature = serde_wasm_bindgen::from_value(full_signature).unwrap();
    let partial = PartialSignature::from(full_signature);
    serde_wasm_bindgen::to_value(&partial).unwrap()
}

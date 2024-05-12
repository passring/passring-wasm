pub mod public {
    use hex::{FromHex, ToHex};
    use passring::traits::Random;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(js_name = PublicKey)]
    pub struct PublicKeyWrapper(pub(crate) passring::PublicKey);

    #[wasm_bindgen(js_class = PublicKey)]
    impl PublicKeyWrapper {
        #[wasm_bindgen(constructor)]
        pub fn new(hex: String) -> PublicKeyWrapper {
            let key = passring::PublicKey::from_hex(hex).unwrap();
            PublicKeyWrapper(key)
        }

        #[wasm_bindgen(js_name = random)]
        pub fn random() -> PublicKeyWrapper {
            let key = passring::PublicKey::random(&mut rand::thread_rng());
            PublicKeyWrapper(key)
        }

        #[wasm_bindgen(js_name = encodeHex)]
        pub fn encode_hex(&self) -> String {
            self.0.encode_hex()
        }

        #[wasm_bindgen(js_name = encodeHexUpper)]
        pub fn encode_hex_upper(&self) -> String {
            self.0.encode_hex_upper()
        }
    }
}

pub mod private {
    use hex::{FromHex, ToHex};
    use passring::traits::Random;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(js_name = PrivateKey)]
    pub struct PrivateKeyWrapper(pub(crate) passring::PrivateKey);

    #[wasm_bindgen(js_class = PrivateKey)]
    impl PrivateKeyWrapper {
        #[wasm_bindgen(constructor)]
        pub fn new(hex: String) -> PrivateKeyWrapper {
            let key = passring::PrivateKey::from_hex(hex).unwrap();
            PrivateKeyWrapper(key)
        }

        #[wasm_bindgen(js_name = random)]
        pub fn random() -> PrivateKeyWrapper {
            let key = passring::PrivateKey::random(&mut rand::thread_rng());
            PrivateKeyWrapper(key)
        }

        #[wasm_bindgen(js_name = getPublicKey)]
        pub fn get_public_key(&self) -> super::public::PublicKeyWrapper {
            super::public::PublicKeyWrapper(self.0.into())
        }

        #[wasm_bindgen(js_name = encodeHex)]
        pub fn encode_hex(&self) -> String {
            self.0.encode_hex()
        }

        #[wasm_bindgen(js_name = encodeHexUpper)]
        pub fn encode_hex_upper(&self) -> String {
            self.0.encode_hex_upper()
        }
    }
}

use aws_lc_rs::{
    rand::SystemRandom,
    signature::{Ed25519KeyPair, KeyPair},
};
use jsonwebtoken_aws_lc::{
    decode, encode, get_current_timestamp, Algorithm, DecodingKey, EncodingKey, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: u64,
}

fn main() {
    let rng = SystemRandom::new();
    let doc = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    let encoding_key = EncodingKey::from_ed_der(doc.as_ref());

    let pair = Ed25519KeyPair::from_pkcs8(doc.as_ref()).unwrap();
    let decoding_key = DecodingKey::from_ed_der(pair.public_key().as_ref());

    let claims = Claims { sub: "test".to_string(), exp: get_current_timestamp() };

    let token = encode(&jsonwebtoken_aws_lc::Header::new(Algorithm::EdDSA), &claims, &encoding_key)
        .unwrap();

    let validation = Validation::new(Algorithm::EdDSA);
    let _token_data = decode::<Claims>(&token, &decoding_key, &validation).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Jot {
        encoding_key: EncodingKey,
        decoding_key: DecodingKey,
    }

    impl Jot {
        fn new() -> Jot {
            let rng = SystemRandom::new();
            let doc = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
            let encoding_key = EncodingKey::from_ed_der(doc.as_ref());

            let pair = Ed25519KeyPair::from_pkcs8(doc.as_ref()).unwrap();
            let decoding_key = DecodingKey::from_ed_der(pair.public_key().as_ref());
            Jot { encoding_key, decoding_key }
        }
    }

    #[test]
    fn test() {
        let jot = Jot::new();
        let claims = Claims { sub: "test".to_string(), exp: get_current_timestamp() };

        let token =
            encode(&jsonwebtoken_aws_lc::Header::new(Algorithm::EdDSA), &claims, &jot.encoding_key)
                .unwrap();

        let validation = Validation::new(Algorithm::EdDSA);
        let token_data = decode::<Claims>(&token, &jot.decoding_key, &validation).unwrap();
        assert_eq!(token_data.claims.sub, "test");
    }
}

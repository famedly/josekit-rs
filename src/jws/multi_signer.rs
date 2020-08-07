use serde_json::{json, Map, Value};

use crate::error::JoseError;
use crate::jws::JwsSigner;

pub struct JwsMultiSigner {
    payload: String,
    signatures: Vec<Value>,
}

impl JwsMultiSigner {
    pub fn new(payload: &[u8]) -> Self {
        JwsMultiSigner {
            payload: base64::encode_config(payload, base64::URL_SAFE_NO_PAD),
            signatures: Vec::new(),
        }
    }

    pub fn add_signature(
        &mut self,
        signer: &dyn JwsSigner,
        protected_header: &Map<String, Value>,
        unprotected_header: &Map<String, Value>,
    ) -> Result<(), JoseError> {
        let protected_header = serde_json::to_string(&protected_header).unwrap();
        let protected_header = base64::encode_config(protected_header, base64::URL_SAFE_NO_PAD);

        let message = format!("{}.{}", protected_header, self.payload);
        let signature = signer.sign(message.as_bytes())?;
        let signature = base64::encode_config(&signature, base64::URL_SAFE_NO_PAD);

        self.signatures.push(json!({
            "protected": protected_header,
            "header": unprotected_header,
            "signature": signature,
        }));

        Ok(())
    }

    pub fn serialize_json(&self) -> Result<String, JoseError> {
        Ok(format!(
            "{{\"payload\":{},\"signatures\":{}}}",
            &serde_json::to_string(&self.payload).unwrap(),
            &serde_json::to_string(&self.signatures).unwrap(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jws::{RS256, ES256};
/*
    #[test]
    fn sign_multpile() -> Result<()> {
        let payload = b"abcde012345";

        let mut multi_signer = JwsMultiSigner::new(payload);

        let keypair1 = RS256.generate_keypair(2048)?;
        let signer1 = RS256.signer_from_der(keypair1.to_der_private_key())?;

        let keypair2 = ES256.generate_keypair()?;

        multi_signer.add_signature(signer: &dyn JwsSigner, protected_header: &Map<String, Value>, unprotected_header: &Map<String, Value>)

        Ok(())
    }
*/
}
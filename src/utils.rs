use crate::KeystoreError;
use starknet_crypto::{get_public_key, FieldElement};
use starknet_ff::FromByteArrayError;

pub fn get_pubkey<T: AsRef<[u8]>>(secret_scalar: T) -> Result<FieldElement, KeystoreError> {
    if secret_scalar.as_ref().len() > 32 {
        return Err(KeystoreError::FieldElementError(FromByteArrayError));
    }

    let sk = unsafe { &*(secret_scalar.as_ref().as_ptr() as *const [u8; 32]) };
    let sk = FieldElement::from_bytes_be(sk).map_err(|e| KeystoreError::FieldElementError(e))?;

    Ok(get_public_key(&sk))
}

use log::{info, trace, warn};
use machineid_crystal::{HWIDComponent, IdBuilder};
use ruscrypt::aes;

const SIGNING_KEY: &str = "impetus-impresario";
const KEY_SIZE: &str = "256";
const ENCRYPTION_MODE: &str = "ECB";
const ENCODING: &str = "base64";
const NULL_CHAR: &str = "\x00";
const NULL_BARRIER: &str = "\x00\x00";

/// Returns the base 64 encoded machine and OS specific encryption of a secret for
/// use in configuration files
pub fn encrypt(secret: &str, key: &str) -> String {
    match secret.find(NULL_CHAR) {
        Some(_) => panic!("The secret cannot contain the Null character (Unicode code point 0x00)"),
        None => {}
    };

    let uuid = uuid::Uuid::new_v4();
    let plain_text = format!("{}{NULL_BARRIER}{}", secret, uuid);

    aes::encrypt(&plain_text, &key, KEY_SIZE, ENCRYPTION_MODE, ENCODING).unwrap()
}

/// Decrypt a password encrypted with a machine and OS specific key and strip the salt
pub fn decrypt(cipher_text: &str, key: &str) -> String {
    let plain_text = aes::decrypt(cipher_text, &key, KEY_SIZE, ENCRYPTION_MODE, ENCODING).unwrap();

    let position: usize = match plain_text.find(NULL_BARRIER) {
        Some(p) => p,
        None => {
            panic!("The encrypted secret is not in the correct format.");
        }
    };
    String::from(&plain_text[0..position])
}

pub fn get_machine_key() -> String {
    let key = generate_key();
    trace!("Key: {key}");
    key
}

fn generate_key() -> String {
    let key = match IdBuilder::new(machineid_crystal::Encryption::SHA256)
        .add_component(HWIDComponent::CPUID)
        .add_component(HWIDComponent::SystemID)
        .add_component(HWIDComponent::DriveSerial)
        .build(Some(SIGNING_KEY))
    {
        Ok(k) => k,
        Err(error) => {
            info!(
                "Unable to generate the key using the complete machine and OS signature. Using the fallback option.\nCause:\n{error:?}"
            );
            generate_fallback_key()
        }
    };
    key
}

fn generate_fallback_key() -> String {
    match IdBuilder::new(machineid_crystal::Encryption::SHA256)
        .add_component(HWIDComponent::CPUID)
        .add_component(HWIDComponent::SystemID)
        .build(Some(SIGNING_KEY))
    {
        Ok(k) => k,
        Err(error) => {
            warn!(
                "Unable to generate the key using the fallback option. Using falminimal option.\n**This option is significantly less secure!**\nCause:\n{error:?}"
            );
            generate_minimal_key()
        }
    }
}

fn generate_minimal_key() -> String {
    match IdBuilder::new(machineid_crystal::Encryption::SHA256)
        .add_component(HWIDComponent::SystemID)
        .build(Some(SIGNING_KEY))
    {
        Ok(k) => k,
        Err(error) => {
            panic!("Unable to generate a secure key. Aborting encryption.\nCause:\n{error:?}");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::encryption::{decrypt, encrypt, get_machine_key};

    #[test]
    /// Test all Unicode characters except null
    fn test_encrypt_decrypt() {
        let mut original_text: String = String::new();
        let key = get_machine_key();

        for c in 0x01..=0xD7FF {
            original_text.push(char::from_u32(c).unwrap());
        }
        for c in 0xE000..0x10FFFF {
            original_text.push(char::from_u32(c).unwrap());
        }

        let cipher_text = encrypt(&original_text, &key);
        let plain_text = decrypt(&cipher_text, &key);

        assert_eq!(original_text, plain_text);
    }
}

use std::process;

use argh::FromArgs;
use impetus_impresario_common::encryption::{self, get_machine_key};

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "secret")]
/// password: actions related to securely handling secrets, such as passwords, with II
pub struct SecretSubcommand {
    #[argh(switch, short = 'e')]
    /// encrypt a secret for secure inclusion in a `config.json` file
    /// If the secrets entered do not match the CLI will exit with code 1
    /// for use in scripts.
    /// Note: the encrypted password is OS and machine specific, and cannot be
    /// decrypted on another installation of the same OS or a different machine
    pub encrypt: bool,
    #[argh(option, short = 'k')]
    /// use a specific key to encrypt the secret
    /// If this option is absent, the machine and OS specific key will be used
    pub key: Option<String>,
}

impl SecretSubcommand {
    pub fn encrypt(&self) {
        println!("Encrypt secret:");

        let password = rpassword::prompt_password("Type the password to encrypt: ").unwrap();
        let repassword = rpassword::prompt_password("Retype the password to encrypt: ").unwrap();

        if password != repassword {
            println!("The passwords entered did not match. Encryption aborted.");
            process::exit(1);
        }

        let key = match &self.key {
            Some(k) => k,
            None => &get_machine_key(),
        };

        println!(
            "Encrypted password: {}",
            encryption::encrypt(&password, &key)
        );
    }
}

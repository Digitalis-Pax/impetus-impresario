use crate::commands::{Subcommand, Subcommands};

mod command_secret;
mod commands;

pub fn run() {
    let subcommand: Subcommand = argh::from_env();

    match subcommand.subcommand {
        Subcommands::Secret(password_subcommand) => {
            if password_subcommand.encrypt {
                password_subcommand.encrypt();
            }
        }
    }
}

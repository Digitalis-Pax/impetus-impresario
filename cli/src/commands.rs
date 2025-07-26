use argh::FromArgs;

use crate::command_secret::SecretSubcommand;

#[derive(FromArgs, PartialEq, Debug)]
/// The Impetus Impresario command provides multiple subcommands, each of which
/// has its own set of options in addition to the common options all subcommands share.
pub struct Subcommand {
    #[argh(subcommand)]
    pub subcommand: Subcommands,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum Subcommands {
    Secret(SecretSubcommand),
}

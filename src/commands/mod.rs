use clap::Subcommand;

pub mod init;

#[derive(Subcommand)]
pub enum Commands {
    Init,
}
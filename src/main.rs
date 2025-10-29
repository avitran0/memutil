use clap::Parser as _;

use crate::{
    args::{Args, Commands},
    commands::{
        find::{find, find_function},
        list::list,
        read::read_once,
        watch::watch,
    },
};

mod args;
mod commands;
mod data_type;
mod memory;
mod signature;
mod value;

fn main() {
    let args = Args::parse();
    match args.command {
        Commands::Read {
            pid,
            signature,
            data_type,
        } => read_once(pid, signature, data_type),
        Commands::Watch {
            pid,
            signature,
            data_type,
            interval,
        } => watch(pid, signature, data_type, interval),
        Commands::Find { pid, signature } => find(pid, signature),
        Commands::FindFunction { pid, function_name } => find_function(pid, function_name),
        Commands::List { pid } => list(pid),
    }
}

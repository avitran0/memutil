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

mod address;
mod args;
mod commands;
mod data_type;
mod memory;
mod value;

fn main() {
    let args = Args::parse();
    match args.command {
        Commands::Read {
            pid,
            address,
            data_type,
        } => read_once(pid, address, data_type),
        Commands::Watch {
            pid,
            address,
            data_type,
            interval,
        } => watch(pid, address, data_type, interval),
        Commands::Find { pid, address } => find(pid, address),
        Commands::FindFunction { pid, function_name } => find_function(pid, function_name),
        Commands::List { pid } => list(pid),
    }
}

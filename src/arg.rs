use clap::{Arg,Parser,Command};

#[derive(Debug,Parser)]
pub struct KshkCli{
    #[arg(short,long)]
    usr_name : String,
    [arg(Command)]
    auth : Command,
}

#[derive(Command)]
pub enum Command{
    
}

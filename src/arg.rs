use std::fmt::format;
use std::net::Ipv4Addr;

use clap::{Arg, Command, Error, Parser,};
use clap::clap_derive;
use std::convert::TryFrom;

use crate::_constw::PORT_RANGE;

#[derive(Debug,Parser)]
pub struct KshkCli{
    
    pub adrr : Option<u16>,
    #[arg(short,long)]
    pub usr_name : String,
    #[arg(short,long)]
    pub auth : bool,
    
    #[arg(short, long)]
    pub buffer_size : Option<u16>,
}

// #[derive(Debug,Clone)]
//  pub enum ParseBuffSize{
//    ByKilloBytes(u64),
//    ByMegaBytes(u64),
//    ByByte(u64),
//    None,
// }



// fn parse_size(s : &str) -> Result<ParseBuffSize,String>{
//     let (num , unit) = s.split_at(s.len() - 2);
//     let size : u64 = num.parse().map_err(|_| format!("invalid number {}",num))?;
//     match unit.to_lowercase().as_str(){
//         "kb" => Ok(ParseBuffSize::ByKilloBytes(size)),
//         "mb" => Ok(ParseBuffSize::ByMegaBytes(size)),
//         "b" => Ok(ParseBuffSize::ByByte(size)),
//         _ => Err(format!("Invalid unit {}",unit)),
//     }
// }



fn parse_adrr(adrr_ : String) -> Result<String,String>{
    let split_adrr : Vec<&str> = adrr_.split(':').collect();
    if PORT_RANGE.contains(&split_adrr[1].parse::<usize>().unwrap()) && split_adrr[0].parse::<Ipv4Addr>().is_ok() {
        return Ok(adrr_);
    }else {
        return Err(format!("couldn't parse the adress, incalid one !"));
    }
}
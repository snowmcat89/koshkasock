use std::{error::Error, sync::Arc};

use clap::error;
use koshkasock::{self, os_detect::get_os_type,socks5::{self, AdrrSock}};
use tokio::{net::{TcpListener}, sync::Mutex};

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>>{

    println!("[*] starting the socks5 on {}...", get_os_type());
    let wsmsock = AdrrSock{adrr:String::from("127.0.0.1:3465")};
    wsmsock.run_sock(Arc::new(Mutex::new(false))).await?;
    Ok(())

}
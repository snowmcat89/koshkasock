use std::{error::Error, net::SocketAddr, sync::Arc, time::Duration};
use tokio::{io::AsyncReadExt, sync::Mutex, time};

use log::{info, warn, error, debug, trace};
use env_logger;

use tokio::net::{TcpListener,TcpStream};
use clap::error;


pub struct ProxyHandleClient{
    pub clients : Arc<Mutex<Vec<SocketAddr>>>,
    pub max_clients : Arc<Mutex<usize>>,
}

impl ProxyHandleClient{

    pub async fn add_client(&self, adrr : SocketAddr)  {
        println!("adding client {} ...",&adrr);
        let client_cloned = self.clients.clone();
        let sz_clients = (client_cloned.lock().await).len();
        /*written ">=" reather than "==" for anymistake in code abt previous tasks / code in general */
        if sz_clients >= *(self.max_clients.clone().lock().await){
            error!("Maximum client number has reached ! ");
        }else{
            (client_cloned.lock().await).push(adrr);
        }
    }

    pub async fn rm_client( &self, adrr_ptr : &SocketAddr){
        println!("removing client {}...",adrr_ptr);
        
    }
}




#[derive(Debug,Clone)]
pub struct AdrrSock{
    pub usr_nm : String,
    pass : String,

}




/* recv first req from client (browser for ex) */
pub struct ReqMethodClient{
    pub ver : u8,
    pub n_method : u8,

}


pub async fn handle_socks_client( mut stream : TcpStream ) -> Result<(), Box<dyn Error>> {
    
    let mut fbuff = [0u8,2];
    let n = match time::timeout(tokio::time::Duration::from_secs(5), stream.read_exact(&mut fbuff)).await{
        Ok(Ok(nsize)) => nsize,
        Ok(Err(err)) => {
            warn!("[!] couldn't handle buffer from client ! error : {}",err);
            return Err(Box::new(err))
        }
        Err(_) => {
            warn!("[!] Operation time out!");
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::TimedOut, "Operation time out")))

        }
    };

    info!("Successfully read {} bytes: {:?}", n, &fbuff[..n]);

    // checking fbuff content
    if  fbuff[0] != 0x05 {
        error!("not sock5 verison ! ");
    }
    let n_methods = fbuff[1] as usize;
    
    let mut meths = vec![0u8;n_methods];
    stream.read_exact(&mut meths).await?;

    


    
    Ok(())
}
    

pub struct socksHandlAPi {}

impl socksHandlAPi{
    pub async fn run_sck5(adrr: String,preg_stop : Arc<Mutex<bool>>,dur:Arc<Mutex<u64>>) -> Result<(), Box<dyn Error>>{
        while !*preg_stop.clone().lock().await{
            let socks_listner = TcpListener::bind(adrr.clone()).await?;
            info!("SOCKS5 proxy listening on 127.0.0.1:1080");
            let duration = {
                let dur_lock = dur.lock().await;
                tokio::time::Duration::from_secs(*dur_lock as u64)
            };


            // steram
            let mut stream = match tokio::time::timeout(/*dur */ duration,
        async move {
            socks_listner.accept().await}).await{
                Ok(restream) => {
                    match restream {
                        Ok(res) => {
                            let (strm,_) = res;
                            strm
                        },
                        Err(err) => {
                            println!("[*] error while trying to bind server {}", err);
                            continue;
                        }
                    }
                }
                Err(err) => {
                    println!("[*] No connection has been recieved");
                    continue;
                }
            };



            // handling other stuffs





        }
        Ok(())
    }
}

use std::{borrow::Borrow, collections::HashMap, error::Error, fmt::format, io::Read, mem::uninitialized, net::SocketAddr, sync::Arc, time::Duration};
use tokio::{io::AsyncReadExt, sync::Mutex, time};
use std::io::stdin;
use tokio::io::{self,AsyncWriteExt};
use log::{info, warn, error, debug, trace};
use env_logger;
use std::net::Incoming;
use tokio::net::{TcpListener,TcpStream};
use clap::error;



pub fn hndl_command() {
    loop{
        let mut cmdln = String::new();
        match std::io::Stdin::read_line(&mut cmdln){
            Ok(bfsize) => {
                
            }
            
            Err(err)
        }
    }
}



pub struct ProxyServerConfig{
    adrr : String,
    auth_supp : Arc<Mutex<bool>>,
}



pub trait ProxyHandleClient {
    fn add_client (&self) ;
}

pub struct WithAuthProxy{ 
    clients : Arc<Mutex<HashMap<String,(String,SocketAddr)>>>,
    max_clients : Arc<Mutex<usize>>,
}

impl WithAuthProxy{
    // add client
    pub async fn add_client(&self, adrr : SocketAddr, user_pass : AdrrSock )
    {
        
        println!("adding client {} ...",adrr);
    let client_cloned: Arc<Mutex<HashMap<String, (String, SocketAddr)>>>= self.clients.clone();
        let sz_clients = (client_cloned.lock().await).len();
        /*written ">=" reather than "==" for anymistake in code abt previous tasks / code in general */
        if sz_clients >= *(self.max_clients.clone().lock().await){
            error!("Maximum client number has reached ! ");
        }else{
            
            (client_cloned.lock().await).insert(format!("CLIENT-{}", user_pass.usr_nm),(user_pass.pass,adrr) );
        }
    }



    // remove client 
    pub async fn rm_client( &self, cln_name : String){

        println!("removing {}...",&cln_name);
        if let Some(cnt_sock_adrr) = (self.clients.clone().lock().await).remove(&cln_name){
            info!(" {} have been remove seccessfuly from adress: {}...",cln_name,cnt_sock_adrr.1);

        }else {
            warn!("cannot remove {} , user is not found ",&cln_name);

        }



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
    



pub struct SocksHandlApi {
    configapi: ProxyServerConfig,
}

impl SocksHandlApi {
    // run our server socks5
    pub async fn run_sck5(&self, preg_stop: Arc<Mutex<bool>>, dur: Arc<Mutex<u64>>) -> Result<(), Box<dyn Error>> {
        while !*preg_stop.clone().lock().await {
            let socks_listner = TcpListener::bind(self.configapi.adrr.clone()).await?;
            println!("SOCKS5 proxy listening on {}", self.configapi.adrr);

            let stream: TcpStream = match tokio::time::timeout(Duration::from_secs(*(dur.lock().await)), async {
                socks_listner.accept().await
            }).await {
                Ok(result) => {
                    match result {
                        Ok((strm, _)) => strm,
                        Err(err) => {
                            println!("[*] Error while trying to bind server: {}", err);
                            continue;
                        }
                    }
                }
                Err(_) => {
                    println!("[*] No connection has been received");
                    continue;
                }
            };

            // handling other stuffs


        }

        Ok(())
    }
}




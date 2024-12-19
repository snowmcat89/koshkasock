use std::{sync::Arc, thread::sleep};

use tokio::{io::AsyncReadExt, sync::Mutex};

pub mod arg;
pub mod os_detect;
pub mod socks5;
pub mod hash;
pub mod csvsck;






pub mod _constw {
    use std::ops::RangeInclusive;



            pub const PORT_RANGE : RangeInclusive<usize> = 1..=65535;


            pub const SOCKS_VERSION_ : u8 = 0x05;
            

            // Accepted server req
            pub const SOCKS_NO_AUTH_ :                  u8 = 0x00;
            pub const SOCKS_USR_PSW_AUTH :              u8 = 0x02;
            pub const SOCKS_NO_MET_ACCPTD:              u8 = 0xFF;


            // client's req DETAIS

            pub const SOCKS_CNT_CONN_REQ:               u8 = 0x01;
            pub const SOCKS_CNT_BIND_REQ:               u8 = 0x02;
            pub const SOCKS_CNT_UDP_ASSO:               u8 = 0x03;


            
            

}


pub enum AuthenticationMeth{
    Some{
        usr_nm : String,
        psswd : String,
    },

    None,
}


#[cfg(test)]
mod tests{
    include!("../test/dbg_test.rs");
    
}




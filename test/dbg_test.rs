


#[cfg(test)]

mod tests{
    #[test]
    pub fn check_dbg(){
        use crate::arg;
        let var = arg::KshkCli{
            nog : 46
        };
    }
}

pub fn nothing(){
    println!("hello");
}

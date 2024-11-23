pub fn get_os_type() -> String{
    if cfg!(target_os = "windows"){
        return String::from("windows_os")
    
    }else if cfg!(target_os = "linux"){
        return String::from("linux_os")

    }else {
        String::from("not supported os")
    }
}

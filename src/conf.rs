use config::Config;
use std::sync::RwLock;
use lazy_static::lazy_static;
use sha256::digest;

// well, just global variables you know, rust don't have that by default, SAFETY IS IMPORTANT 💯 💯 💯 
lazy_static! {
    pub static ref DEVICE_NAME: RwLock<String> = RwLock::new(String::new());
    pub static ref DESCRIPTION: RwLock<String> = RwLock::new(String::new());
    pub static ref ACCESS_KEY: RwLock<String> = RwLock::new(String::new());
    pub static ref PRODUCTION: RwLock<bool> = RwLock::new(false);
}

// giving the configuration variables to thos global ones
pub fn init_conf(){
    let config = Config::builder()
        .add_source(config::File::with_name("sitesavior.toml"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();
    
    *DEVICE_NAME.write().unwrap() = config.get("device_name").unwrap();
    *DESCRIPTION.write().unwrap() = config.get("description").unwrap();
    *PRODUCTION.write().unwrap() = config.get("production").unwrap();

    // for extra security, gonna encrypt the access key, i don't wanna anyone access my potato
    let access_key: String = config.get("access_key").unwrap();
    *ACCESS_KEY.write().unwrap() = digest(access_key);
    println!("ACCESS_KEY value: {}", *ACCESS_KEY.read().unwrap());
}
// tbh i feel old when writing variables in CAPITALE, it give me back the good ol' php days 👴
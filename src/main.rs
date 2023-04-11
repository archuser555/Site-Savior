// Copyright 2044-2089, milkway galaxy, credit to Anass zakar 👴👴👴
// Beta 0.1v, it lack a lot of features i wanna to controle my server, but anyways it's beta

#[macro_use]
extern crate rocket;
use rocket::response::content;
use rocket::http::CookieJar;
use std::process::Command;
//use std::collections::HashMap;
//use serde_json::{json, Value};
use crate::conf::{ACCESS_KEY, DEVICE_NAME, DESCRIPTION, PRODUCTION};
use sha256::digest;
mod conf;
mod client;


fn token_check(token: &str) -> bool{
    let access_key = &ACCESS_KEY.read().unwrap() as &str;

    // EXTREA SECURITY 💯💯💯, SHA256 MAKES ME FEEL SECURE!
    if digest(token) == access_key {
        return true
    }
    false
}

fn execute_command(cmd: &str) -> String {
    let output = Command::new(cmd)
        .output()
        .expect("failed to execute process");
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[get("/?<key>")]
fn key(key: &str) -> &str {
    
    if token_check(key) {
        return "access granted"
    }

    return "unknown"
}

#[get("/?<token>&<cmd>")]
fn commands(token: &str, cmd: Option<String>) -> String {
    
    if token_check(token) {
        let cmd = match cmd {
            Some(c) => c,
            None => "ls".to_string(),
        };
        let output = execute_command(&cmd);
        return output.trim().to_string();
    } else {
        return "Invalid token".to_string();
    }
}

#[get("/?<token>")]
fn info(token: &str) -> String {
    if token_check(token) {
        // TODO
        /*let disk = disk_info().unwrap();
        let mem = mem_info().unwrap();
        let load = loadavg().unwrap();    
        let mut info: HashMap<&str, String> = HashMap::new();
        info.insert("name", DEVICE_NAME.read().unwrap().to_string());
        info.insert("desc", DESCRIPTION.read().unwrap().to_string());
        info.insert("disk", format!("{} GB available", disk.free / 1000 / 1000));
        info.insert("ram", format!("{} GB used of {}", mem.total - mem.free, mem.total));
        info.insert("uptime", format!("{}", load.one));
        return serde_json::to_string(&info).unwrap();*/
        "help me get sys info :)".to_string()
    }
    else {
        return "Invalid token".to_string();
    }
}

#[get("/")]
fn index() -> content::RawHtml<String> {
    // This function is in client.rs, to keep things organized
    return content::RawHtml(client::main_page());
}

#[get("/")]
fn dashboard(cookies: &CookieJar<'_>) -> content::RawHtml<String> {
    let cookie = cookies.get("access");
    if token_check(cookie.map(|c| c.value()).unwrap()) {
        // in client.rs, it's just html anyways
        return content::RawHtml(client::dashboard());
    }

    else {
        return content::RawHtml("<script>window.location.href = '/';</script>".to_string()) // redirect with javascript
    }
}

#[launch]
fn rocket() -> _ {
    conf::init_conf();

    rocket::build()
        .mount("/exec", routes![commands])
        .mount("/", routes![index])
        .mount("/api/access", routes![key])
        .mount("/dashboard", routes![dashboard])
        .mount("/api/info", routes![info])
}

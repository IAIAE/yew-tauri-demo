#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use rand::prelude::*;
use serde::{Deserialize, Serialize};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![hello, getUser])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn hello(name: &str) -> Result<String, String> {
  // This is a very simplistic example but it shows how to return a Result
  // and use it in the front-end.
  if name.contains(' ') {
    Err("Name should not contain spaces".to_string())
  } else {
    Ok(format!("Hello, {}", name))
  }
}



#[derive(Deserialize, Serialize, Debug)]
pub struct Home {
  at: f64,
  lo: f64,
  desc: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct User {
  name: String,
  age: u8,
  address: Option<Home>,
}

#[tauri::command]
fn getUser() -> Result<User, String> {
  let mut rng = rand::thread_rng();
  let f: f64 = rng.gen();
  if f > 0.5 {
    Ok(User {
      name: "richcao".to_owned(),
      age: 32,
      address: Some(Home {
        at: 12312.123123,
        lo: 1243342.123123,
        desc: "广州越秀区西华路乐华街7号".to_owned(),
      })
    })
  }else {
    Err("失败概率是50%".into())
  }
}
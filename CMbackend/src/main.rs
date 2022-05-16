#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use inputbot::KeybdKey::*;
use std::{collections::hash_map::DefaultHasher, hash::Hasher, path::Path, time::Duration};
#[macro_use]
extern crate rusqlite;
use rusqlite::Connection;

use arboard::Clipboard;

fn main() -> ! {
    let mut conn = match Connection::open("clipboard") {
        Ok(conn) => conn,
        Err(_) => std::process::exit(1),
    };

    conn.execute(
        "CREATE TABLE IF NOT EXISTS clipboard (
        content     TEXT

    )",
        params![],
    )
    .unwrap();

    let mut clipboard = match Clipboard::new() {
        Ok(clipboard) => clipboard,
        Err(_) => std::process::exit(1),
    };

    loop {
        //open CM
        if OtherKey(0x5B).is_pressed() && CKey.is_toggled() {
            std::thread::sleep(Duration::from_millis(5000));
        }

        //copy into custom buffer

        if LControlKey.is_pressed() && CKey.is_pressed() {
            // std::thread::sleep(Duration::from_millis(500));
            match clipboard.get_text() {
                Ok(content) =>{
                    conn.execute("INSERT INTO clipboard (content) VALUES (?1)", params![content]).unwrap();
                },
                Err(_) => {println!("something wrong")},
            };
            
            

            std::thread::sleep(Duration::from_millis(1000));
        }
        if OtherKey(0x5B).is_pressed() && EscapeKey.is_pressed() {
            std::process::exit(0);
        }
    }
}

use std::process::{id, self};
use eframe::{App, Frame};
use eframe::run_native;
use eframe::egui::{self, ScrollArea, Button};
use eframe::egui::{CentralPanel, Context, Pos2, Vec2};
use eframe::egui::Key::{Escape};
use rusqlite::{Connection, params};
use inputbot::MouseCursor;
fn main() {
    let mouse = MouseCursor::pos();
    let asd:(f32,f32) =( mouse.0 as f32,mouse.1 as f32);
    println!("{:?}", mouse);
    let mut options = eframe::NativeOptions::default();
    options.initial_window_size = Option::from(Vec2 { x: 302., y: 340. });
    options.always_on_top = true;
    options.resizable = false;
    options.initial_window_pos = Some(Pos2{x: asd.0, y: asd.1});
    // options.decorated = false;
    eframe::run_native(
        "CM",
        options,
        Box::new(|_| Box::new(CM::new())),
    );
}



struct CM{
    contents: Vec<String>
}

impl CM {
    fn new()->Self{
        let mut vec = Vec::new();
        let mut conn = match Connection::open("clipboard") {
            Ok(conn) => {conn},
            Err(_) => {process::exit(1)},
        };
        let mut stmt = conn.prepare("SELECT content FROM clipboard").unwrap();
        
        let mut rows = stmt.query([]).unwrap();
        while let Some(row) = rows.next().unwrap() {
            vec.push(row.get(0).unwrap());
        }

       
        CM{contents:vec}
    }
}


impl App for CM{
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx,|ui|{
            
            for i in &self.contents {
                ui.group(|ui|{
                    let formated_text = format!("{}                                                                                   ",&i);
                    // ui.button(&formated_text[0..80]).;
                    ui.add(Button::new(&formated_text[0..80]));
                });
            }


            // ScrollArea::new([false,true]).auto_shrink(Default::default()).show(ui, |ui|{
            //     for elem in &self.contents {
            //         ui.label(elem);
            //         ui.
            //     }
            // });

        });

    }
}


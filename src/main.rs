#[macro_use]
extern crate serde_derive;
use imgui::*;
use nfd::Response;
use std::*;
mod ui;
mod zookeeper;
use ui::*;
const CLEAR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

fn main() {
    let mut state = ui::State::default();
    ui::run("hello.rs".to_owned(), CLEAR_COLOR, hello_world, state);
}

fn hello_world<'a>(ui: &Ui<'a>, state: &mut ui::State) -> bool {
    let mut menu_option = MenuOption::None;
    //let mut draw_list = ui.get_window_draw_list();
    ui.window(im_str!("Hello world"))
        .title_bar(false)
        .position((0.0, 0.0), ImGuiCond::FirstUseEver)
        //      .movable(false)
        .collapsible(false)
        //.always_auto_resize(true)
        .resizable(false)
        .scroll_bar(false)
        .menu_bar(true)
        .size((1024f32, 768f32), ImGuiCond::FirstUseEver)
        .build(|| {
            ui.menu_bar(|| {
                ui.menu(im_str!("File")).build(|| {
                    if ui
                        .menu_item(im_str!("Open"))
                        //                        .shortcut(im_str!("CTRL+O"))
                        .build()
                    {
                        menu_option = MenuOption::Open;
                    };
                    ui.menu_item(im_str!("Metrics"))
                        .selected(&mut state.show_metrics)
                        .build();
                    if ui.menu_item(im_str!("Exit")).build() {
                        state.quit = true;
                        //running = false;
                    };
                });
            });
            if state.show_metrics {
                ui.show_metrics_window(&mut state.show_metrics);
            }
            match menu_option {
                MenuOption::Open => {
                    let result = nfd::dialog().filter("yml").open().unwrap_or_else(|e| {
                        panic!(e);
                    });
                    match result {
                        Response::Okay(file_path) => {
                            use std::fs::File;
                            use std::io::prelude::Read;
                            use std::io::BufReader;
                            let file = File::open(file_path).expect("Failed to Open File");
                            let mut buf_reader = BufReader::new(file);
                            let mut contents = String::new();
                            buf_reader
                                .read_to_string(&mut contents)
                                .expect("Failed to Read File");
                            //state.yml_str = Some(ImString::new(contents))
                            state.yml_str = Some(contents)
                        }
                        Response::Cancel => println!("User canceled"),
                        _ => (),
                    }
                }

                _ => {}
            }

            //            ui.columns(2, im_str!("Main"), true);
            //           ui.separator();
            if state.yml_str.is_some() {
                //ui.input_text_multiline(
                println!(
                    "Avail/max {:?}/{:?}",
                    ui.get_content_region_avail(),
                    ui.get_content_region_max()
                );
                //         println!("Count:{:?}", ui.get_columns_count());
                //        println!("Index:{:?}", ui.get_column_index());
                //       println!("offset{:?}", ui.get_column_offset(1));
                //      println!("Width{:?}", ui.get_column_width(1));
                println!("Cursor{:?}", ui.get_cursor_pos());
                println!("ScreenCursor{:?}", ui.get_cursor_screen_pos());

                //let mut draw_list = ui.get_window_draw_list();
                //            ui.new_line();
                /*                let canvas_pos = ui.get_cursor_screen_pos();
                //let canvas_pos = (ui.get_column_offset(1), 150.);
                let canvas_size = //(512., 768.);
                ui.get_content_region_avail();
                const CANVAS_CORNER_COLOR1: [f32; 3] = [1., 0.2, 0.2];
                const CANVAS_CORNER_COLOR2: [f32; 3] = [1., 0.2, 0.24];
                const CANVAS_CORNER_COLOR3: [f32; 3] = [1., 0.24, 0.27];
                const CANVAS_CORNER_COLOR4: [f32; 3] = [1., 0.2, 0.24];
                draw_list.add_rect_filled_multicolor(
                    canvas_pos,
                    (canvas_pos.0 + canvas_size.0, canvas_pos.1 + canvas_size.1),
                    CANVAS_CORNER_COLOR1,
                    CANVAS_CORNER_COLOR1,
                    CANVAS_CORNER_COLOR1,
                    CANVAS_CORNER_COLOR1,
                );*/
                //zookeeper::add_rects(ui, state, &mut draw_list);
                zookeeper::add_rects(ui, state);
                ui.text(
                    //   im_str!(""),
                    &mut ImString::new(state.yml_str.clone().unwrap()),
                    //(512., 768.),
                )
                //.no_horizontal_scroll(true)
                //.read_only(true)
                //.build();
            }

            //          ui.next_column();
            // UI AREA
        });

    true
}

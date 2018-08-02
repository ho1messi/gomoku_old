extern crate backend;

#[macro_use]
extern crate sciter;

/*
pub use sciter::dom::Element;
pub use sciter::dom::event::EventHandler;
pub use sciter::host::Archive;
pub use sciter::host::Host;
pub use sciter::host::HostHandler;
pub use sciter::value::Value;
pub use sciter::value::FromValue;
pub use sciter::window::Window;
*/

use sciter::host::*;
use sciter::graphics::Image;
use sciter::dom::Element;
use sciter::window::Window;
use sciter::value::Value;
use sciter::HELEMENT;

use std::collections::HashMap;

struct EventHandle {
    root: Option<Element>,
}

impl EventHandle {
    fn native_call(&mut self, arg: String) -> Value {
        println!("Be called by script!");
        return Value::from(format!("get value back, {}", arg));
    }
}

impl sciter::dom::event::EventHandler for EventHandle {
    fn attached(&mut self, root: HELEMENT) {
        self.root = Some(Element::from(root));
    }

    dispatch_script_call! {
        fn native_call(String);
    }
}

struct LoadHandle {
    uris: Vec<String>,
    data_map: HashMap<String, &'static [u8]>,
}

impl LoadHandle {
    fn new() -> LoadHandle {
        return LoadHandle {
            uris: Vec::new(),
            data_map: HashMap::new(),
        };
    }

    fn add_data(&mut self, uri: &str, data: &'static [u8]) {
        self.data_map.insert(String::from(uri), data);
        self.uris.push(String::from(uri));
    }
}

impl sciter::host::HostHandler for LoadHandle {
    fn on_data_load(&mut self, pnm: &mut sciter::host::SCN_LOAD_DATA) -> Option<LOAD_RESULT> {
        let uri_t = w2s!(pnm.uri);
        for uri in self.uris.iter() {
            println!("uri: {}", *uri);
            println!("uri_t: {}", uri_t);
            if uri_t.contains(uri) {
                if let Some(data) = self.data_map.get(uri) {
                    self.data_ready(pnm.hwnd, &uri_t, data, None);
                    break;
                }
            }
        }

        return Some(LOAD_RESULT::LOAD_DEFAULT);
    }
}

fn main() {
    let event_handler = EventHandle{root: None};
    let mut load_handle = LoadHandle::new();
    let mut frame = sciter::WindowBuilder::main_window().with_size((750, 600)).fixed().create();

    let gomoku_ico = include_bytes!("../resources/gomoku.ico");
    let board_img = include_bytes!("../resources/board.jpg");
    let white_chess_img = include_bytes!("../resources/white.png");
    let black_chess_img = include_bytes!("../resources/black.png");
    let gomoku_tis = include_bytes!("../resources/gomoku.tis");
    load_handle.add_data("gomoku.ico", gomoku_ico);
    load_handle.add_data("board.jpg", board_img);
    load_handle.add_data("white.png", white_chess_img);
    load_handle.add_data("black.png", black_chess_img);
    load_handle.add_data("gomoku.tis", gomoku_tis);

    let html = include_bytes!("../resources/gomoku.htm");
    frame.event_handler(event_handler);
    frame.sciter_handler(load_handle);
    frame.load_html(html, Some("resources://gomoku.htm"));
    frame.run_app();
}

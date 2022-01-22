use std::thread;
use std::time::Duration;
use x11rb::connect;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{AtomEnum, ConnectionExt, MapState, Screen};
use x11rb::rust_connection::RustConnection;

fn main() {
    println!("Press Ctrl+C to stop the program");

    // connect to x11 display
    let (conn, screen_num) = connect(None).unwrap();
    let screen = &conn.setup().roots[screen_num];

    loop {
        unmap_line_borders(&conn, &screen);

        //prevent cpu hogging
        thread::sleep(Duration::from_secs(1));
    }
}

fn unmap_line_borders(conn: &RustConnection, screen: &Screen) {

    // get the windows tree from root screen
    let root = conn.query_tree(screen.root)
        .expect("Fail to get cookie")
        .reply()
        .expect("ReplyError for QueryTree");

    // check all the sub-windows available atm
    for window in root.children {

        // skip unmapping operation if the window is already unmapped
        // or fail to get the attribute
        let process_attribute = conn.get_window_attributes(window)
            .expect("Can't get the cookie")
            .reply();

        match process_attribute {
            Ok(reply) => {
                if reply.map_state.eq(&MapState::UNMAPPED) {
                    continue;
                }
            }
            Err(_) => {
                continue;
            }
        }


        // get the window name and title
        let process_name = conn.get_property(
            false,
            window,
            AtomEnum::WM_CLASS,
            AtomEnum::STRING,
            0,
            u32::MAX,
        )
            .expect("Can't get cookie")
            .reply()
            .expect("Can't get WM_CLASS property from window");

        let process_title = conn.get_property(
            false,
            window,
            AtomEnum::WM_NAME,
            AtomEnum::STRING,
            0,
            u32::MAX,
        )
            .expect("Can't get cookie")
            .reply()
            .expect("Can't get WM_NAME property from window");


        // convert from vec to string
        let process_name = String::from_utf8(process_name.value).expect("Can't convert bytes to string for process name");
        let process_title = String::from_utf8(process_title.value).expect("Can't convert bytes to string for process title");

        // only unmap window related to line program and with empty title
        if (process_name.contains("line.exe") || process_name.contains("linemediaplayer.exe")) && process_title == "" {
            match conn.unmap_window(window) {
                Ok(_) => {
                    println!("Unmap line window with id : {:#0x}", window)
                }
                Err(_) => {
                    println!("Fail to unmap line window with id : {:#0x}", window)
                }
            }
        }
    }
}

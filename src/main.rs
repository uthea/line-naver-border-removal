use x11rb::connect;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{AtomEnum, ConnectionExt, get_property, query_tree};

fn main() {
    let (conn, screen_num) = connect(None).unwrap();
    let screen = &conn.setup().roots[screen_num];

   let result=  query_tree(&conn, screen.root)
       .unwrap()
       .reply();

    match result {
        Ok(val)=> {
            for item in val.children {
                let process = get_property(&conn, false, item, AtomEnum::WM_CLASS, AtomEnum::STRING, 0, u32::MAX).unwrap().reply();
                let process_title = get_property(&conn, false, item, AtomEnum::WM_NAME, AtomEnum::STRING, 0, u32::MAX).unwrap().reply().expect("can't get process title");

                match process {
                    Ok(reply) => {
                        let process_name = String::from_utf8(reply.value).expect("Can't parse");
                        let process_title = String::from_utf8(process_title.value).expect("Can't parse");

                        if (process_name.contains("line.exe") || process_name.contains("linemediaplayer.exe")) &&  process_title == "" {
                            // println!("Window with id {:#0x}, name = {}", item, process_name);
                            conn.unmap_window(item).expect("Can't unmap the window");
                        }
                    },
                    Err(_) => {
                        println!("Error can't get process name");
                    }
                }
            }
        }

        Err(_)=> {
            println!("Error");
        }
    }
}



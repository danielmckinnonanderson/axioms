use std::{
    borrow::BorrowMut,
    collections::HashMap,
    fs::File,
    io::{self, Read},
    thread,
};

use messaging::MessageType;
use rouille::{router, try_or_400, websocket, Response};

mod game;
mod messaging;

const SERVER_HOSTNAME: &'static str = "localhost";
const SERVER_PORT: u16 = 8080;

fn main() {
    // A room is an instance of a game -- Players can join and leave rooms.
    let _rooms: HashMap<u16, ()> = HashMap::new();
    let _clients: Vec<String> = vec![];

    let server_addr = format!("{}:{}", SERVER_HOSTNAME, SERVER_PORT);
    println!("Listening on {server_addr}");

    rouille::start_server(server_addr, move |request| {
        router!(request,
            (GET) (/) => {
                let client_html_f = File::open("src/bin-form.html").expect("Couldn't find client HTML");
                let client_html: String = {
                    let mut buf = String::new();

                    io::BufReader::new(client_html_f)
                        .read_to_string(buf.borrow_mut())
                        .expect("Couldn't read contents from client.html");

                    buf
                };
                Response::html(client_html.as_str())
            },
           (GET) (/ws) => {
                let (response, websocket) = try_or_400!(websocket::start(request, Some("axioms")));

                // Spawn a separate thread for each websocket.
                thread::spawn(move || {
                    // Block until response resolves.
                    let ws = websocket.recv().unwrap();
                    websocket_handling_thread(ws);
                });

                response
            },
            _ => rouille::Response::empty_404()
        )
    });
}

fn websocket_handling_thread(mut websocket: websocket::Websocket) {
    // We wait for a new message to come from the websocket.
    while let Some(message) = websocket.next() {
        match message {
            websocket::Message::Text(txt) => {
                // If the message is text, send it back with `send_text`.
                println!("received {:?} from a websocket", txt);
                websocket.send_text(&txt).unwrap();
            }
            websocket::Message::Binary(data) => {
                let maybe_msg: Result<MessageType, ()> = data.try_into();
                if let Err(_) = maybe_msg {
                    return;
                }

                let msg = maybe_msg.unwrap();
                println!("{:?}", msg);
            }
        }
    }
}

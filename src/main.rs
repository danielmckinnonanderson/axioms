use std::{borrow::BorrowMut, collections::HashMap, fs::File, io::{self, BufRead, Read}, thread};

use messaging::MessageType;
use rouille::{input::json, router, try_or_400, websocket, Request, RequestBody, Response, ResponseBody};

mod cards;
mod game;
mod messaging;

const SERVER_HOSTNAME: &'static str = "localhost";
const SERVER_PORT: u16 = 8080;


fn main() {
    // A room is an instance of a game -- Players can join and leave rooms.
    let rooms: HashMap<u16, ()> = HashMap::new();
    let clients: Vec<String> = vec![];

    let server_addr = format!("{}:{}", SERVER_HOSTNAME, SERVER_PORT);
    println!("Listening on {server_addr}");

    let client_html_f = File::open("src/client.html").expect("Couldn't find client HTML");
    let client_html: String = {
        let mut buf = String::new();
        
        io::BufReader::new(client_html_f)
            .read_to_string(buf.borrow_mut())
            .expect("Couldn't read contents from client.html");

        buf
    };


    rouille::start_server(server_addr, move |request| {
        router!(request,
            (GET) (/) => {
                Response::html(client_html.as_str())
            },
           (GET) (/ws) => {
                // This is the websockets route.

                // In order to start using websockets we call `websocket::start`.
                // The function returns an error if the client didn't request websockets, in which
                // case we return an error 400 to the client thanks to the `try_or_400!` macro.
                //
                // The function returns a response to send back as part of the `start_server`
                // function, and a `websocket` variable of type `Receiver<Websocket>`.
                // Once the response has been sent back to the client, the `Receiver` will be
                // filled by rouille with a `Websocket` object representing the websocket.
                let (response, websocket) = try_or_400!(websocket::start(request, Some("echo")));

                // Because of the nature of I/O in Rust, we need to spawn a separate thread for
                // each websocket.
                thread::spawn(move || {
                    // This line will block until the `response` above has been returned.
                    let ws = websocket.recv().unwrap();
                    // We use a separate function for better readability.
                    websocket_handling_thread(ws);
                });

                response
            },
            _ => rouille::Response::empty_404()
        )
    });
}

// Function run in a separate thread.
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

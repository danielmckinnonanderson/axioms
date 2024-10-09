use std::{
    borrow::BorrowMut, collections::HashMap, fs::File, io::{self, Read}, sync::{Arc, Mutex}, thread
};

use game::GameState;
use messaging::MessageType;
use rouille::{router, try_or_400, websocket, Response};

use crate::messaging::{try_parse_message, DeserializationError};

mod game;
mod messaging;

const SERVER_HOSTNAME: &'static str = "localhost";
const SERVER_PORT: u16 = 8080;

pub type RoomId = u64;

fn main() {
    let server_addr = format!("{}:{}", SERVER_HOSTNAME, SERVER_PORT);
    println!("Listening on {server_addr}");

    let game_state: Arc<Mutex<GameState>> = Arc::new(Mutex::new(GameState::default()));

    rouille::start_server(server_addr, move |request| {
        let mut game_state = Arc::clone(&game_state);
        router!(request,
            (GET) (/) => {
                let client_html_f = File::open("src/bin-form.html")
                    .expect("Couldn't find client HTML");

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
                    websocket_handling_thread(ws, &mut game_state);
                });

                response
            },
            _ => rouille::Response::empty_404()
        )
    });
}

fn websocket_handling_thread(
    mut websocket: websocket::Websocket,
    app_state: &mut Arc<Mutex<GameState>>,
) {
    let id = rand::random::<u64>();
    let connected_reponse = websocket.send_binary(&id.to_be_bytes());
    if connected_reponse.is_err() {
        eprintln!("{:?}", connected_reponse);
        return;
    }

    // We wait for a new message to come from the websocket.
    while let Some(message) = websocket.next() {
        match message {
            websocket::Message::Text(txt) => {
                // If the message is text, send it back with `send_text`.
                println!("received {:?} from a websocket", txt);
                websocket.send_text(&txt).unwrap();
            }
            websocket::Message::Binary(data) => {
                let maybe_msg: Result<MessageType, DeserializationError> = try_parse_message(data);
                if maybe_msg.is_err() {
                    eprintln!("Could not deserialize message: {:?}", maybe_msg);
                    return;
                }

                let msg = maybe_msg.unwrap();
                println!("Received {:?}", msg);

                app_state.lock()
                    .expect("Could not acquire lock on game state")
                    .transition(msg);

                println!("Evaluated and transitioned state");

                // For now send the client the ID of their thread
                // TODO - Formalize a "message receipt confirmation" response structure
                match websocket.send_binary(&id.to_be_bytes()) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error sending message: {:?}", e);
                    }
                }
            }
        }
    }
}

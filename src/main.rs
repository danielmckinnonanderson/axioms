use std::{collections::HashMap, thread};

use rouille::{input::json, router, try_or_400, websocket, Request, RequestBody, Response, ResponseBody};

mod cards;
mod game;

const SERVER_HOSTNAME: &'static str = "localhost";
const SERVER_PORT: u16 = 8080;


fn main() {
    // A room is an instance of a game -- Players can join and leave rooms.
    let rooms: HashMap<u16, ()> = HashMap::new();
    let clients: Vec<String> = vec![];

    let server_addr = format!("{}:{}", SERVER_HOSTNAME, SERVER_PORT);
    println!("Listening on {server_addr}");

    rouille::start_server(server_addr, move |request | {
        router!(request,
            (GET) (/) => {
                Response::html("<script type=\"text/javascript\">
                    var socket = new WebSocket(\"ws://localhost:8080/ws\", \"echo\");
                    function send(data) {{
                        socket.send(data);
                    }}
                    socket.onmessage = function(event) {{
                        document.getElementById('result').innerHTML += event.data + '<br />';
                    }}
                    </script>
                    <p>This example sends back everything you send to the server.</p>
                    <p><form onsubmit=\"send(document.getElementById('msg').value); return false;\">
                    <input type=\"text\" id=\"msg\" />
                    <button type=\"submit\">Send</button>
                    </form></p>
                    <p>Received: </p>
                    <p id=\"result\"></p>")
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
            websocket::Message::Binary(_) => {
                println!("received binary from a websocket");
            }
        }
    }
}

import {MessageClient} from "../../backend/bindings/MessageClient"
import {MessageServer} from "../../backend/bindings/MessageServer"

console.log("Hello ts")

const socket = new WebSocket("ws://localhost:8080")

// Connection opened
socket.addEventListener("open", (event) => {
    let message: MessageClient = {
        Connection: "Pseudo"
    }
    socket.send(JSON.stringify(message));
  });

// Listen for messages
socket.addEventListener("message", (event) => {
  let message: MessageServer = JSON.parse(event.data);
  console.log("Message from server ", message);
});
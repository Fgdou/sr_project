import {MessageClient} from "../../backend/bindings/MessageClient"

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
console.log("Message from server ", event.data);
});
import { Infos } from "../../backend/bindings/Infos";
import {MessageClient} from "../../backend/bindings/MessageClient"
import {MessageServer} from "../../backend/bindings/MessageServer"
import {Canvas} from "./Canvas.js"

function main(){
  
}
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

  if ("Infos" in message)
    draw(message["Infos"])
});

let html = (document.getElementById("canvas") as HTMLCanvasElement)
let size = {
  x: html.width,
  y: html.height
}
let canvas = new Canvas(html.getContext("2d")!, size)

function draw(message: Infos) {
  canvas.clear()
  canvas.drawGrid(message.size)
}
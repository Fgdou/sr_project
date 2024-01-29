import { Direction } from "../../backend/bindings/Direction";
import { Infos } from "../../backend/bindings/Infos";
import {MessageClient} from "../../backend/bindings/MessageClient"
import {MessageServer} from "../../backend/bindings/MessageServer"
import {Canvas} from "./Canvas.js"

const socket = new WebSocket(`ws://${location.hostname}:8080`)

// Connection opened
socket.addEventListener("open", (event) => {
  let pseudo = window.prompt("Username") as string;
  let message: MessageClient = {
      Connection: pseudo
  }
  socket.send(JSON.stringify(message));
});

// Listen for messages
socket.addEventListener("message", (event) => {
  let message: MessageServer = JSON.parse(event.data);

  if ("Infos" in message)
    draw(message["Infos"])
});

function keyHandler(event: KeyboardEvent) {
  let code = event.key;

  var dir: Direction|undefined = undefined;
  switch (code) {
    case "ArrowLeft": dir = "Left"; break;
    case "ArrowRight": dir = "Right"; break;
    case "ArrowUp": dir = "Up"; break;
    case "ArrowDown": dir = "Down"; break;
    default:
  }

  if(dir != undefined) {
    let message: MessageClient = {
      ChangeDirection: dir
    }
    socket.send(JSON.stringify(message))
  }
}

window.addEventListener("keydown", keyHandler)

let html = (document.getElementById("canvas") as HTMLCanvasElement)
let size = {
  x: html.width,
  y: html.height
}
let canvas = new Canvas(html.getContext("2d")!, size)

function draw(message: Infos) {
  canvas.clear()
  message.apples.forEach(apple => canvas.drawRectangle(apple, message.size, "red"))
  message.players.forEach(player => {
    player.positions.forEach(p => canvas.drawRectangle(p, message.size, "gray"))
  })
  canvas.drawGrid(message.size)
}
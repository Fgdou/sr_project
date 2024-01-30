import { Direction } from "../../backend/bindings/Direction";
import { Infos } from "../../backend/bindings/Infos";
import {MessageClient} from "../../backend/bindings/MessageClient"
import {MessageServer} from "../../backend/bindings/MessageServer"
import {Canvas} from "./Canvas.js"
import { getSocket, getUsername } from "./utils.js";

let protocol = (location.protocol == "https:") ? "wss" : "ws"
let urls = [
  `${protocol}://${location.hostname}/ws`,
  `${protocol}://${location.hostname}:8080`
]

let socket: WebSocket|undefined = undefined;


(async () => {
  for(let url of urls){
    try{
      socket = await getSocket(url)

      // Listen for messages
      socket.addEventListener("message", (event) => {
        let message: MessageServer = JSON.parse(event.data);

        console.log(message)
    
        if ("Infos" in message)
          draw(message["Infos"])
      });
    
      // Connection opened
      let pseudo = getUsername();
      console.log(`Hello ${pseudo}`)
      let message: MessageClient = {
          Connection: pseudo
      }
      socket?.send(JSON.stringify(message));
    
      break
    } catch (e) {}
  }
})()

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
    socket?.send(JSON.stringify(message))
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
    canvas.drawPlayer(player, message.size)
  })
  canvas.drawGrid(message.size)
}
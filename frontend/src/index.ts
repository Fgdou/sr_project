import { Direction } from "../../backend/bindings/Direction";
import { Infos } from "../../backend/bindings/Infos";
import {MessageClient} from "../../backend/bindings/MessageClient"
import {MessageServer} from "../../backend/bindings/MessageServer"
import {Canvas} from "./Canvas.js"
import { getPlayer, getSocket, getUsername, setupKeyboard, setupSwipes } from "./utils.js";

let protocol = (location.protocol == "https:") ? "wss" : "ws"
let urls = [
  `${protocol}://${location.hostname}/ws`,
  `${protocol}://${location.hostname}:8080`
]

let socket: WebSocket|undefined = undefined;
let id: number|undefined = undefined;

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
        if ("SetId" in message)
          id = message["SetId"]
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

let html = (document.getElementById("canvas") as HTMLCanvasElement)
let size = {
  x: html.width,
  y: html.height
}
let canvas = new Canvas(html.getContext("2d")!, size)

let divUsername = document.getElementById("username") as HTMLSpanElement
let divScore = document.getElementById("score") as HTMLSpanElement

setupSwipes(dir => {
  let message: MessageClient = {
    "ChangeDirection": dir
  }
  socket?.send(JSON.stringify(message))
})
setupKeyboard(dir => {
  let message: MessageClient = {
    ChangeDirection: dir
  }
  socket?.send(JSON.stringify(message))
})

function draw(message: Infos) {
  canvas.clear()
  message.apples.forEach(apple => canvas.drawRectangle(apple, message.size, "red"))
  message.players.forEach(player => {
    canvas.drawPlayer(player, message.size, player.id == id)
  })
  canvas.drawGrid(message.size)

  let player = getPlayer(message, id)
  if(player){
    divUsername.textContent = player.username
    divScore.textContent = player.positions.length.toString()
  }
}
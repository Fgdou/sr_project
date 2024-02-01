import { Direction } from "../../backend/bindings/Direction";
import { Infos } from "../../backend/bindings/Infos";
import {MessageClient} from "../../backend/bindings/MessageClient"
import {MessageServer} from "../../backend/bindings/MessageServer"
import {Canvas} from "./Canvas.js"
import { Leaderboard } from "./leaderboard.js";
import { getPlayer, getSocket, getUsername, setupKeyboard, setupSwipes } from "./utils.js";

let protocol = (location.protocol == "https:") ? "wss" : "ws"
let urls = [
  `${protocol}://${location.hostname}/ws`,
  `${protocol}://${location.hostname}:8080`
]

let socket: WebSocket|undefined = undefined;
let id: number|undefined = undefined;
let leaderboard = new Leaderboard("leaderboard");

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
        if ("Error" in message)
          handleError(message.Error)
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

export function handleError(error: string) {
  alert(error)
  window.logout()
}

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
  canvas.setGridSize(message.size)

  // apples
  message.apples.forEach(apple => canvas.drawRectangle(apple, "red"))

  // players
  message.players.filter(p => p.id != id).forEach(player => {
    canvas.drawPlayer(player, false)
  })
  message.players.filter(p => p.id == id).forEach(player => {
    canvas.drawPlayer(player, true)
  })

  // grid
  canvas.drawGrid()

  // player names
  message.players
    .filter(p => p.id != id && p.state == "Running")
    .forEach(player => {
      let head = player.positions[player.positions.length-1]
      canvas.drawText(player.username, head, "white")
    })

  // show info on the current player
  let player = getPlayer(message, id)
  if(player){
    divUsername.textContent = player.username
    divScore.textContent = player.positions.length.toString()

    if(player.state instanceof Object && "Waiting" in player.state && player.state.Waiting%2 == 0) {
      let pos = {
        x: message.size.x/2,
        y: message.size.y*0.75
      }
      canvas.drawText((player.state.Waiting/2).toString(), pos, "rgba(255, 255, 255, .3)", 30);
    }
  }

  // draw leaderboard
  leaderboard.update(message, id)
}

declare global {
  interface Window {logout: () => void}
}

window.logout = () => {
  document.cookie = "username=;expires=Thu, 01 Jan 1970 00:00:00 GMT";
  location.reload()
}
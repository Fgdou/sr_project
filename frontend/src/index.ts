import { Infos } from "../../backend/bindings/Infos";
import {Canvas} from "./Canvas.js"
import { Client } from "./Client.js";
import { Game } from "./Game.js";
import { registerLoginCallback } from "./LoginWindow.js";
import { Leaderboard } from "./leaderboard.js";
import { getPlayer, getUsername, setupKeyboard, setupSwipes } from "./utils.js";

let username = getUsername()

if(username != undefined) {
  startGame(username)
} else {
  registerLoginCallback(startGame)
}


function startGame(username: string) {
  document.cookie = `username=${username}`
  document.getElementById("login")?.classList.remove("open")
  document.getElementById("game")?.classList.add("open")

  let leaderboard = new Leaderboard("leaderboard");

  let html = (document.getElementById("canvas") as HTMLCanvasElement)
  let size = {
    x: html.width,
    y: html.height
  }
  let canvas = new Canvas(html.getContext("2d")!, size)

  let divUsername = document.getElementById("username_name") as HTMLSpanElement
  let divScore = document.getElementById("score") as HTMLSpanElement

  let game: Game|undefined = undefined;
  let client = new Client(infos => {
    game = new Game(infos)
  }, change => {
    change.forEach(c => game?.update(c))
    game?.tick()


    let infos = game?.getInfos()
    if(infos != undefined)
      draw(infos)
  }, username);



  setupSwipes(dir => {
    client.sendMessage({
      "ChangeDirection": dir
    })
  })
  setupKeyboard(dir => {
    client.sendMessage({
      ChangeDirection: dir
    })
  })

  function draw(message: Infos) {
    canvas.clear()
    canvas.setGridSize(message.size)
  
    // apples
    message.apples.forEach(apple => canvas.drawRectangle(apple, "red"))
  
    // players
    let first = (message.players.length != 0) ? message.players.sort((a, b) => b.positions.length - a.positions.length)[0].id : undefined
    message.players.filter(p => p.id != client.getId()).forEach(player => {
      canvas.drawPlayer(player, false, first == player.id)
    })
    message.players.filter(p => p.id == client.getId()).forEach(player => {
      canvas.drawPlayer(player, true, first == player.id)
    })
  
    // player names
    message.players
      .filter(p => p.id != client.getId() && p.state == "Running")
      .forEach(player => {
        let head = player.positions[player.positions.length-1]
        canvas.drawText(player.username, head, "white")
      })
  
    // show info on the current player
    let player = getPlayer(message, client.getId())
    if(player){
      divUsername.innerHTML = `<b>${player.username}</b> (${Math.floor(client.averagePing())}ms)`
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
    leaderboard.update(message, client.getId())
  }
}

declare global {
  interface Window {logout: () => void}
}

window.logout = () => {
  document.cookie = "username=;expires=Thu, 01 Jan 1970 00:00:00 GMT";
  location.reload()
}
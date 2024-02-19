import { Event } from "../../backend/bindings/Event";
import { Infos } from "../../backend/bindings/Infos";
import {Canvas} from "./Canvas.js"
import { Client } from "./Client.js";
import { Deadscreen } from "./Deadscreen.js";
import { ErrorBanner } from "./ErrorBanner.js";
import { Game } from "./Game.js";
import { registerLoginCallback } from "./LoginWindow.js";
import { Leaderboard } from "./leaderboard.js";
import { getPlayer, getUsername, setupKeyboard, setupSwipes, getParams } from "./utils.js";

let error = getParams('error')
if(error != undefined) {
  console.log(error)
  new ErrorBanner(error)
}

let username = getUsername()
let time = 0;
let client_handle: Client|undefined = undefined

if(username != undefined) {
  startGame(username)
} else {
  registerLoginCallback(startGame)
}

function startGame(username: string) {
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

  let info_callback = (infos: Infos) => {
    game = new Game(infos)
    draw(infos)
  };
  let change_callback = (change: Event[]) => {
    change.forEach(c => game?.update(c))
    game?.tick()

    let infos = game?.getInfos()
    if(infos != undefined)
      draw(infos)
  }

  let client = new Client(info_callback, change_callback, username, leaderboard);
  client_handle = client

  document.cookie = `username=${username}`
  document.getElementById("login")?.classList.remove("open")
  document.getElementById("game")?.classList.add("open")
  document.getElementsByClassName("canvas")[0].classList.add("animate")
  setTimeout(() => document.getElementsByClassName("canvas")[0].classList.remove("animate"), 2000)



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

    let player = getPlayer(message, client.getId())

    if (player?.state == "Running") {
      time += 300
    }
  
    // apples
    message.apples.forEach(apple => canvas.drawRectangle(apple, "red"))
  
    // players
    let first = (message.players.length != 0) ? message.players.sort((a, b) => b.positions.length - a.positions.length)[0].id : undefined
    message.players.filter(p => p.id != client.getId() && !(p.state instanceof Object && 'Dead' in p.state && p.state.Dead == 0)).forEach(player => {
      canvas.drawPlayer(player, false, first == player.id)
    })
    if(player != undefined)
      canvas.drawPlayer(player, true, first == player.id)
  
    // player names
    message.players
      .filter(p => p.id != client.getId() && p.state == "Running")
      .forEach(player => {
        let head = player.positions[player.positions.length-1]
        canvas.drawText(player.username, head, "white")
      })
  
    // show info on the current player
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
  
    // test dead
    if(player != undefined && player.state instanceof Object && "Dead" in player.state && player.state.Dead == 12) {
      new Deadscreen(player, message.players, time/1000)
    }
  }
}

declare global {
  interface Window {logout: (error: string|undefined) => void}
}

window.logout = (error) => {
  document.cookie = "username=;expires=Thu, 01 Jan 1970 00:00:00 GMT";
  client_handle?.disconnect();
  if (error == undefined)
    location.reload()
  else
    location.replace(`?error=${error}`)
}
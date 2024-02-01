import { Event } from "../../backend/bindings/Event";
import { Infos } from "../../backend/bindings/Infos";
import { Vector2 } from "../../backend/bindings/Vector2";

export class Game{
    constructor(private infos: Infos) {

    }
    update(message: Event) {
        if ("RemoveApple" in message) {
            let apple = message.RemoveApple
            this.infos.apples = this.infos.apples.filter(a => a.x != apple.x && a.y != apple.y)
        }
        if ("AddApple" in message) {
            this.infos.apples.push(message.AddApple)
        }
        if ("AddPlayer" in message) {
            this.infos.players.push(message.AddPlayer)
        }
        if ('IncreasePlayer' in message) {
            this.infos.players.filter(p => p.id == message.IncreasePlayer).forEach(p => p.positions.unshift(p.positions[0]))
        }
        if ('RemovePlayer' in message) {
            this.infos.players = this.infos.players.filter(p => p.id != message.RemovePlayer)
        }
        if ('MovePlayer' in message) {
            this.infos.players.filter(p => p.id == message.MovePlayer.id).forEach(p => p.direction = message.MovePlayer.dir)
        }
        if ('ChangeStatePlayer' in message) {
            this.infos.players.filter(p => p.id == message.ChangeStatePlayer.id).forEach(p => p.state = message.ChangeStatePlayer.state)
        }
    }
    tick() {
        this.infos.players.forEach(p => {
            if(p.state != "Running") return;

            let dir = {x: 0, y: -1}
            if(p.direction == 'Down') {
                dir = {x: 0, y: 1}
            }
            if(p.direction == 'Left') {
                dir = {x: -1, y: 0}
            }
            if(p.direction == 'Right') {
                dir = {x: 1, y: 0}
            }

            let pos = p.positions[p.positions.length-1]
            let newPos = {x: pos.x+dir.x, y: pos.y+dir.y}

            p.positions.push(newPos)
            p.positions.shift()
        })
    }
    getInfos(): Infos {
        return this.infos
    }

}
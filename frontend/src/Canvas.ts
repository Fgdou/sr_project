import { Player } from "../../backend/bindings/Player";
import { Vector2 } from "../../backend/bindings/Vector2";

export class Canvas {
    private playerPos: Vector2 = {x: 0, y: 0}

    constructor(private canvas: CanvasRenderingContext2D, private canvasSize: Vector2, private cellSize: Vector2){
    }

    setPlayerPos(pos: Vector2) {
        this.playerPos = pos
    }

    drawRectangle(pos: Vector2, color: string) {
        this.canvas.fillStyle = color
        pos = this.worldToScreen(pos)
        this.canvas.fillRect(pos.x, pos.y, this.cellSize.x, this.cellSize.y)
    }
    clear() {
        this.canvas.reset()
    }
    drawGrid() {
        this.canvas.beginPath()
        this.canvas.strokeStyle = "#000000"
        this.canvas.lineWidth = 1

        for(let i=0; i<=40; i++) {
            let x = this.canvasSize.x/2 + (-20+i)*this.cellSize.x
            this.canvas.moveTo(x, 0)
            this.canvas.lineTo(x, this.canvasSize.y)
        }
        for(let i=0; i<=40; i++) {
            let y = this.canvasSize.y/2 + (-20+i)*this.cellSize.y
            this.canvas.moveTo(0, y)
            this.canvas.lineTo(this.canvasSize.x, y)
        }

        this.canvas.stroke()
        this.canvas.closePath()
    }
    drawBondaries(gameSize: Vector2) {
        this.canvas.beginPath()
        this.canvas.strokeStyle = "#ffffff"
        this.canvas.lineWidth = 1

        let upLeft = this.worldToScreen({x: 0, y: 0})
        let bottomRight = this.worldToScreen(gameSize)

        this.canvas.moveTo(upLeft.x, upLeft.y)
        this.canvas.lineTo(upLeft.x, bottomRight.y)
        this.canvas.lineTo(bottomRight.x, bottomRight.y)
        this.canvas.lineTo(bottomRight.x, upLeft.y)
        this.canvas.lineTo(upLeft.x, upLeft.y)

        this.canvas.stroke()
        this.canvas.closePath()
    }
    drawPlayer(player: Player, me: boolean) {
        let color = "gray"
        if (player.state == "Running") {
            color = (me) ? "#0dce74" : "#864AF9"
        }

        player.positions.forEach(p => {
            this.drawRectangle(p, color)
        })

        if(player.state instanceof Object && "Waiting" in player.state && !me) {
            let pos = player.positions[player.positions.length-1]
            this.drawText(Math.floor(player.state.Waiting/2+0.5).toString(), {x: pos.x, y: pos.y+1}, "white", 1.7);
        }
    }
    drawText(text: string, pos: Vector2, color: string, fontSize: number = 1.0) {
        this.canvas.fillStyle = color
        this.canvas.font = `bold ${15*fontSize}px sans-serif`
        this.canvas.textAlign = "center"

        pos.x += 0.5
        pos.y -= .1
        
        pos = this.worldToScreen(pos)

        this.canvas.fillText(text, pos.x, pos.y)
    }
    drawTextCenter(text: string, color: string, fontSize: number = 1.0) {
        this.canvas.fillStyle = color
        this.canvas.font = `bold ${15*fontSize}px sans-serif`
        this.canvas.textAlign = "center"

        let pos = {
            x: this.canvasSize.x*0.5,
            y: this.canvasSize.y
        }
        
        this.canvas.fillText(text, pos.x, pos.y)
    }
    private worldToScreen(pos: Vector2): Vector2 {
        return {
            x: (pos.x - this.playerPos.x)*this.cellSize.x + this.canvasSize.x/2,
            y: (pos.y - this.playerPos.y)*this.cellSize.y + this.canvasSize.y/2,
        }
    }
}
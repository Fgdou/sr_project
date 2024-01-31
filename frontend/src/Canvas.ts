import { Player } from "../../backend/bindings/Player";
import { Vector2 } from "../../backend/bindings/Vector2";

export class Canvas {
    gridSize: Vector2 = {x: 1, y: 1}
    playerPos: Vector2 = {x: 0, y: 0}

    constructor(private canvas: CanvasRenderingContext2D, private size: Vector2){
    }

    setGridSize(size: Vector2) {
        this.gridSize = size
    }
    drawRectangle(pos: Vector2, color: string) {
        let width = this.size.x/this.gridSize.x;
        let height = this.size.y/this.gridSize.y;

        this.canvas.fillStyle = color
        this.canvas.fillRect(pos.x*width, pos.y*height, width, height)
    }
    clear() {
        this.canvas.reset()
    }
    drawGrid() {
        this.canvas.strokeStyle = "#000000"
        this.canvas.lineWidth = 1

        let width = this.size.x/this.gridSize.x;
        let height = this.size.y/this.gridSize.y;

        for(let i=0; i<=this.gridSize.x; i++) {
            this.canvas.moveTo(width*i, 0)
            this.canvas.lineTo(width*i, this.size.y)
        }
        for(let i=0; i<=this.gridSize.x; i++) {
            this.canvas.moveTo(0, i*height)
            this.canvas.lineTo(this.size.x, i*height)
        }
        this.canvas.stroke()
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
        let width = this.size.x/this.gridSize.x;
        let height = this.size.y/this.gridSize.y;

        this.canvas.fillText(text, (pos.x+0.5)*width, (pos.y-.1)*height)
    }
}
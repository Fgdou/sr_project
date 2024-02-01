import { Player } from "../../backend/bindings/Player";
import { Vector2 } from "../../backend/bindings/Vector2";

export class Canvas {
    gridSize: Vector2 = {x: 1, y: 1}
    playerPos: Vector2 = {x: 0, y: 0}

    crownImgElement: HTMLImageElement
    deadImgElement: HTMLImageElement

    constructor(private canvas: CanvasRenderingContext2D, private size: Vector2){
        this.crownImgElement = document.getElementById("crown") as HTMLImageElement
        this.deadImgElement = document.getElementById("dead") as HTMLImageElement
    }

    setGridSize(size: Vector2) {
        this.gridSize = size
    }
    drawRectangle(pos: Vector2, color: string) {
        let width = this.size.x/this.gridSize.x;
        let height = this.size.y/this.gridSize.y;

        this.canvas.fillStyle = color
        this.canvas.fillRect(pos.x*width+2, pos.y*height+2, width-4, height-4)
    }
    clear() {
        this.canvas.reset()
    }
    drawPlayer(player: Player, me: boolean, first: boolean) {
        let color = "gray"
        if (player.state == "Running") {
            color = "#864AF9"
        }
        if(me && !(player.state instanceof Object && "Dead" in player.state)) {
            color = "#0dce74"
        }

        player.positions.forEach(p => {
            this.drawRectangle(p, color)
        })

        if(player.state instanceof Object && "Waiting" in player.state) {
            let pos = player.positions[player.positions.length-1]
            this.drawText(Math.floor(player.state.Waiting/2+0.5).toString(), {x: pos.x, y: pos.y+1}, "white", 1.7);
        }

        if(player.state instanceof Object && "Dead" in player.state)
            this.drawImage('dead', player.positions[player.positions.length-1], 'white')
        if(first && player.state == "Running") 
            this.drawImage('crown', player.positions[player.positions.length-1], 'white')
    }
    drawText(text: string, pos: Vector2, color: string, fontSize: number = 1.0) {
        this.canvas.fillStyle = color
        this.canvas.font = `bold ${15*fontSize}px sans-serif`
        this.canvas.textAlign = "center"
        let width = this.size.x/this.gridSize.x;
        let height = this.size.y/this.gridSize.y;

        this.canvas.fillText(text, (pos.x+0.5)*width, (pos.y-.2)*height)
    }
    private getImage(name: string): CanvasImageSource {
        if(name == 'dead') return this.deadImgElement
        if(name == 'crown') return this.crownImgElement
        throw `image ${name} not found`
    }
    drawImage(name: string, pos: Vector2, color: string) {
        let width = this.size.x/this.gridSize.x;
        let height = this.size.y/this.gridSize.y;

        this.canvas.fillStyle = color

        this.canvas.drawImage(this.getImage(name), pos.x*width, pos.y*height, width, height)
    }
}
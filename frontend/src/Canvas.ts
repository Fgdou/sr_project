import { Vector2 } from "../../backend/bindings/Vector2";

export class Canvas {
    constructor(private canvas: CanvasRenderingContext2D, private size: Vector2){
    }

    drawRectangle(pos: Vector2, size: Vector2, color: string) {
        this.canvas.fillStyle = color
        this.canvas.fillRect(pos.x, pos.y, this.size.x/size.x, this.size.y/size.y)
    }
    clear() {
        this.canvas.clearRect(0, 0, this.size.x, this.size.y)
    }
    drawGrid(size: Vector2) {
        this.canvas.strokeStyle = "#000000"

        let width = this.size.x/size.x;
        let height = this.size.y/size.y;

        for(let i=0; i<=size.x; i++) {
            this.canvas.moveTo(width*i, 0)
            this.canvas.lineTo(width*i, this.size.y)
        }
        for(let i=0; i<=size.x; i++) {
            this.canvas.moveTo(0, i*height)
            this.canvas.lineTo(this.size.x, i*height)
        }
        this.canvas.stroke()
    }
}
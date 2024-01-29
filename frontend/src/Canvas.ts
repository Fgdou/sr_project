import { Vector2 } from "../../backend/bindings/Vector2";

export class Canvas {
    constructor(private canvas: CanvasRenderingContext2D, private size: Vector2){
    }

    drawRectangle(pos: Vector2, size: Vector2, color: string) {
        let width = this.size.x/size.x;
        let height = this.size.y/size.y;

        this.canvas.fillStyle = color
        this.canvas.fillRect(pos.x*width, pos.y*height, width, height)
    }
    clear() {
        this.canvas.reset()
    }
    drawGrid(size: Vector2) {
        this.canvas.strokeStyle = "#000000"
        this.canvas.lineWidth = 2

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
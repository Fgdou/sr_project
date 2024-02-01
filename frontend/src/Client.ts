import { Infos } from "../../backend/bindings/Infos.js";
import { MessageClient } from "../../backend/bindings/MessageClient.js";
import { MessageServer } from "../../backend/bindings/MessageServer.js";
import { getSocket, getUsername } from "./utils.js";

let protocol = (location.protocol == "https:") ? "wss" : "ws"
let urls = [
    `${protocol}://${location.hostname}/ws`,
    `${protocol}://${location.hostname}:8080`
  ]

export class Client {
    private socket: WebSocket | undefined = undefined
    private id: number|undefined = undefined
    private last = 0
    private delays: number[] = []

    constructor(private callback: (message: Infos) => void) {
        (async () => {
            for(let url of urls){
                try{
                    this.socket = await getSocket(url)
                
                    // Listen for messages
                    this.socket?.addEventListener("message", (event) => {
                        let message: MessageServer = JSON.parse(event.data);
                
                        console.log(message)
                    
                        if ("Infos" in message)
                            this.handleMessage(message.Infos)
                        if ("SetId" in message)
                            this.id = message["SetId"]
                        if ("Error" in message)
                            this.handleError(message.Error)
                    });
                    
                    // Connection opened
                    let pseudo = getUsername();
                    console.log(`Hello ${pseudo}`)
                    this.sendMessage({
                        Connection: pseudo
                    })
                    this.last = Date.now()
                    break
                } catch (e) {}
            }
        })()
    }

    sendMessage(message: MessageClient) {
        this.socket?.send(JSON.stringify(message))
    }
    getId(): number|undefined {
        return this.id
    }

    private handleError(error: string) {
        alert(error)
        window.logout()
    }
    averagePing(): number {
        if (this.delays.length == 0) return 0
        return this.delays.map(n => n/this.delays.length).reduce((prev, n) => prev+n)
    }
    private handleMessage(message: Infos) {
        let now = Date.now()

        let ping = Math.max(now-this.last-300, 0)
        this.last = now
        let diff = this.averagePing()*2-ping

        if (ping > 10)
        this.delays.push(ping)

        diff = Math.max(0, diff)


        setTimeout((m: Infos) => this.callback(m), diff, message)
    }
}
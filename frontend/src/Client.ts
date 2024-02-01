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
    private timings: number[] = []

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
                            setTimeout((m: Infos) => this.handleMessage(m), Math.random()*300, message.Infos);
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
                    this.timings.push(Date.now())
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
    private averageTiming(): number {
        if(this.timings.length < 2) return 0

        let times = 0
        for(let i=1; i<this.timings.length; i++){
            times += (this.timings[i]-this.timings[i-1])/(this.timings.length-1)
        }
        return times
    }
    private handleMessage(message: Infos) {
        let now = Date.now()        
        let average = this.averageTiming()
        let next = this.timings[this.timings.length-1]+average

        let diff = next-now

        diff = Math.max(0, diff)

        console.log(`Average ${average}, Wating ${diff}`)
        this.timings.push(now)


        setTimeout(() => this.callback(message), diff)
    }
}
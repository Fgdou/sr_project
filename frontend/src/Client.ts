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
                            callback(message["Infos"])
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
}
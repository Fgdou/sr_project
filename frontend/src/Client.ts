import { Event } from "../../backend/bindings/Event.js";
import { Infos } from "../../backend/bindings/Infos.js";
import { MessageClient } from "../../backend/bindings/MessageClient.js";
import { MessageServer } from "../../backend/bindings/MessageServer.js";
import { MessageTPSSmoother } from "./MessageTPSSmoother.js";
import { getSocket } from "./utils.js";

let protocol = (location.protocol == "https:") ? "wss" : "ws"
let urls = [
    `${protocol}://${location.hostname}/ws`,
    `${protocol}://${location.hostname}:8080`
  ]

export class Client {
    private socket: WebSocket | undefined = undefined
    private id: number|undefined = undefined
    private messageHandler: MessageTPSSmoother<MessageServer>
    private count = 0;

    constructor(callbackInfos: (message: Infos) => void, callbackChanges: (Message: Event[]) => void, username: string) {
        this.messageHandler = new MessageTPSSmoother(infos => {
            if ('ChangeInfos' in infos){
                let count = infos.ChangeInfos.count

                if (count != this.count) {
                    this.sendMessage("ResendAll")
                } else {
                    callbackChanges(infos.ChangeInfos.events)
                }
                this.count += 1
            }
        }, false);

        (async () => {
            for(let url of urls){
                try{
                    this.socket = await getSocket(url)
                
                    // Listen for messages
                    this.socket?.addEventListener("message", (event) => {
                        let message: MessageServer = JSON.parse(event.data);
                
                        console.log(message)
                    
                        if ("Infos" in message){
                            this.count = message.Infos.message_count
                            callbackInfos(message.Infos)
                        }
                        if ("ChangeInfos" in message){
                            this.messageHandler.call(message)
                        }
                        if ("SetId" in message)
                            this.id = message["SetId"]
                        if ("Error" in message)
                            this.handleError(message.Error)
                    });
                    
                    // Connection opened
                    console.log(`Hello ${username}`)
                    this.sendMessage({
                        Connection: username
                    })
                    break
                } catch (e) {}
            }
        })()
    }

    sendMessage(message: MessageClient) {
        console.log(message)
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
        return this.messageHandler.averagePing()
    }
}
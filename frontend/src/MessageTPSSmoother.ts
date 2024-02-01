export class MessageTPSSmoother<T> {
    private last = 0
    private delays: number[] = []

    constructor(private callback: (message: T) => void, private debug: boolean = false) {
        this.last = Date.now()
    }
    call(message: T) {
        if(this.debug) {
            setTimeout(MessageTPSSmoother.handleMessage<T>, Math.random()*300, this, message)
        } else {
            MessageTPSSmoother.handleMessage(this, message)
        }
    }
    averagePing(): number {
        if (this.delays.length == 0) return 0
        return this.delays.map(n => n/this.delays.length).reduce((prev, n) => prev+n)
    }
    private static handleMessage<T>(handler: MessageTPSSmoother<T>, message: T) {
        let now = Date.now()

        let ping = Math.max(now-handler.last-300, 0)
        handler.last = now
        let diff = handler.averagePing()*1.5 - ping

        if (ping > 10)
        handler.delays.push(ping)

        diff = Math.max(0, diff)

        handler.delays = handler.delays.slice(handler.delays.length-10, handler.delays.length)


        setTimeout(handler.callback, diff, message)
    }
}
export class MessageTPSSmoother<T> {
    private last = 0
    private delays: number[] = []

    constructor(private callback: (message: T) => void, private debug: boolean = false) {
        this.last = Date.now()
    }
    call(message: T) {
        if(this.debug) {
            setTimeout(MessageTPSSmoother.handleMessage<T>, Math.random()*400, this, message)
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
        let diff = handler.averagePing()*2 - ping

        handler.delays.push(ping)

        diff = Math.max(0, diff)

        handler.delays = handler.delays.slice(handler.delays.length-50, handler.delays.length)


        setTimeout(handler.callback, diff, message)
    }
}
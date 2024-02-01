export class ErrorBanner{
    constructor(message: string) {
        let div = document.createElement('div')

        div.innerText = message
        div.classList.add("error")

        document.getElementsByTagName("body")[0].appendChild(div)

        setTimeout(() => {
            div.classList.add("open")
        }, 100)
        setTimeout(() => {
            div.classList.remove("open")
        }, 5000)
        setTimeout(() => {
            document.getElementsByTagName("body")[0].removeChild(div)
            window.history.pushState(null, "", "/")
        }, 7000)
    }
}
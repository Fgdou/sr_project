export class Diapo {
    login: HTMLDivElement

    constructor() {
        this.login = document.getElementById("login") as HTMLDivElement
        document.getElementById("pres")!.addEventListener("click", e => {
            this.open()
        })
    }

    is_open(): boolean {
        return this.login.classList.contains("diapo")
    }

    open() {
        this.login.classList.add("diapo")
    }
    close() {
        this.login.classList.remove("diapo")
    }
}
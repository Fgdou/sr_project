export function registerLoginCallback(callback: (username: string) => void) {
    let element = document.getElementById("username") as HTMLFormElement
    element.addEventListener("submit", (event: SubmitEvent) => {
        event.preventDefault()

        let data = new FormData(element)
        let username = data.get("username")

        if(username){
            callback(username?.toString())
            document.getElementById("login")?.classList.remove("open")
        }
    })
}
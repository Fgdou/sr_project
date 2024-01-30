export function getCookie(cname: string): string {
    let name = cname + "=";
    let decodedCookie = decodeURIComponent(document.cookie);
    let ca = decodedCookie.split(';');
    for(let i = 0; i <ca.length; i++) {
      let c = ca[i];
      while (c.charAt(0) == ' ') {
        c = c.substring(1);
      }
      if (c.indexOf(name) == 0) {
        return c.substring(name.length, c.length);
      }
    }
    return "";
}
export function getUsername(): string {
    let username = getCookie("username")

    while(username == "") {
        username = window.prompt("Username") as string
        document.cookie = `username=${username}`
    }

    return username
  }
  
export function getSocket(url: string): Promise<WebSocket> {
    return new Promise((resolve, reject) => {
        let socket = new WebSocket(url)
        socket.addEventListener("open", e => {
        resolve(socket)
        })
        socket.addEventListener("error", e => {
        reject()
        })
    })
}
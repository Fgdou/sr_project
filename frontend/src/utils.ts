import { Direction } from "../../backend/bindings/Direction";
import { Infos } from "../../backend/bindings/Infos";
import { Player } from "../../backend/bindings/Player";

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
export function getUsername(): string|undefined {
    let username = getCookie("username")

    if(username == "") return undefined

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
export function getPlayer(infos: Infos, id: number|undefined): Player|undefined {
    if(id == undefined) return undefined
    let list = infos.players.filter(p => p.id == id)
    if (list.length == 0) return undefined
    return list[0]
}

export function setupSwipes(callback: (dir: Direction) => void) {
    document.addEventListener('touchstart', handleTouchStart, false);        
    document.addEventListener('touchmove', handleTouchMove, false);

    var xDown: number|null = null;                                                        
    var yDown: number|null = null;

    function getTouches(evt: TouchEvent) {
        return evt.touches
    }                                                     
                                                                            
    function handleTouchStart(evt: TouchEvent) {
        const firstTouch = getTouches(evt)[0];                                      
        xDown = firstTouch.clientX;                                      
        yDown = firstTouch.clientY;                                      
    };                                                
                                                                            
    function handleTouchMove(evt: TouchEvent) {
        evt.preventDefault()
        if ( ! xDown || ! yDown ) {
            return;
        }

        var xUp = evt.touches[0].clientX;                                    
        var yUp = evt.touches[0].clientY;

        var xDiff = xDown - xUp;
        var yDiff = yDown - yUp;
                                                                            
        if ( Math.abs( xDiff ) > Math.abs( yDiff ) ) {/*most significant*/
            if ( xDiff < 0 ) {
                callback("Right")
            } else {
                callback("Left")
            }                       
        } else {
            if ( yDiff < 0 ) {
                callback("Down")
            } else { 
                callback("Up")
            }                                                                 
        }
        /* reset values */
        xDown = null;
        yDown = null;                                             
    };
}
export function setupKeyboard(callback: (dir: Direction) => void) {
    function keyHandler(event: KeyboardEvent) {
        let code = event.key;
      
        var dir: Direction|undefined = undefined;
        switch (code) {
          case "ArrowLeft": dir = "Left"; break;
          case "ArrowRight": dir = "Right"; break;
          case "ArrowUp": dir = "Up"; break;
          case "ArrowDown": dir = "Down"; break;
          default:
        }
      
        if(dir != undefined) {
          event.preventDefault()
          callback(dir)
        }
      }
      
      window.addEventListener("keydown", keyHandler)
}
export function getParams(name: string): string|undefined{
    let search = (new RegExp('[?&]'+encodeURIComponent(name)+'=([^&]*)')).exec(location.search)
    if( search )
       return decodeURIComponent(search[1]);
    return undefined
 }
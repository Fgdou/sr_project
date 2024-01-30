import { Infos } from "../../backend/bindings/Infos"

export class Leaderboard {
    div: HTMLTableElement

    constructor(id: string) {
        this.div = document.getElementById(id) as HTMLTableElement
    }

    update(message: Infos, player_id: number|undefined) {
        let table = message.players.sort((a, b) => b.positions.length - a.positions.length).map(p => {
            let tr = this.div.insertRow()
            let td1 = tr.insertCell()
            let td2 = tr.insertCell()
            td1.textContent = p.positions.length.toString()
            td2.textContent = p.username

            if(p.id == player_id) {
                tr.classList.add("active")
            }

            tr.appendChild(td1)
            tr.appendChild(td2)

            return tr
        })
        this.div.textContent = ""
        table.forEach(t => this.div.appendChild(t))
    }
}
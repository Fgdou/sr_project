import { Infos } from "../../backend/bindings/Infos"

export class Leaderboard {
    div: HTMLTableElement

    constructor(id: string) {
        this.div = document.getElementById(id) as HTMLTableElement
    }

    update(leaderboard: Array<[string, number]>, player_name: string|undefined) {
        this.div.textContent = ""
        leaderboard.sort((a, b) => b[1] - a[1]).map(([username, score]) => {
            let tr = this.div.insertRow()
            let td1 = tr.insertCell()
            let td2 = tr.insertCell()
            td1.textContent = score.toString()
            td2.textContent = username

            if(player_name == username) {
                tr.classList.add("active")
            }
        })
    }
}
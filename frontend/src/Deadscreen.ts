import { Player } from "../../backend/bindings/Player";

export class Deadscreen {
    constructor(player: Player, players: Player[], time: number) {
        (document.getElementById("retry") as HTMLDivElement).classList.add("open");
        document.getElementsByClassName("canvas")[0].classList.add("animate")
        let score = player.positions.length;
        let bestPlayer = players.sort((p1, p2) => p2.positions.length - p1.positions.length)[0];

        let table = document.getElementById("retry-table") as HTMLTableElement
        
        this.addToTable(table, "My Score", score.toString())
        this.addToTable(table, "Best Player", `${bestPlayer.username} (${bestPlayer.positions.length.toString()})`)
        this.addToTable(table, "Time Played", `${time.toString()}s`)
    }

    addToTable(table: HTMLTableElement, name: string, value: string) {
        let row = table.insertRow()
        let cell1 = row.insertCell()
        let cell2 = row.insertCell()

        cell1.textContent = name
        cell2.textContent = value
    }
}
Design
===

# Idea

![snake example](./snake-example.jpg)

The idea is to create a snake game, but multiplayer. The idea is to copy the classical game :
- a fixed size grid
- only one apple for everyone
- movement every time $t$
- every apple increment the size of the player

# Architecture
The architecture is **frontend - backend**. The project can be run with a `docker-compose` file that runs everything. The pipeline allows the build and test of the project at every commit, and deploy the main branch.

The backend is written in **Rust**, and the frontend in **Typescript**. They communicate via **WebSocket**.


```mermaid
---
title: Architecture
---
flowchart TB

subgraph Docker
Nginx[[Nginx :80]] -- / --> Frontend
Nginx -- /ws --> Backend

Frontend -.-> Objects
Backend -.-> Objects
end

Client -- http --> Nginx
LoadTesting -- Websocket --> Backend
```

```mermaid
---
title: Interactions
---
sequenceDiagram

participant Frontend
participant Backend

note over Backend,Frontend: Initialization

Frontend ->> Backend: Initiate Websocket Connection
Frontend ->> Backend: Username

Backend ->> Frontend: ID
Backend ->> Frontend: State of the board

note over Backend,Frontend: Every 300ms

Backend ->> Backend: Game Loop
Backend ->> Frontend: Changes

note over Backend,Frontend: User Input

Frontend ->> Frontend: User Input
Frontend ->> Backend: Direction
```

## Backend
Rust is usefull to have a performant backend. It is also very handy for a lot of usecase in this project, because it is  a functional programming language. Many `enum` are used to transfer data accross the front and backend.

The Backend handles all the game logic. The position and the direction is stored here. It receives the command from the users for the direction, and first sends the position of everyone and the apple.

## Frontend
Typscript will be the easiest for frontend development but will still allow for type checking. I chose to not work with any frameworks, because this is a simple frontend.

The frontend will draw the players sent by the server and send every keys sent by the user. It will also execute the game logic for the client, as the server will only send basic modifications of the game : 
- change direction of the player
- new player
- eating an apple

## Link between Frontend and Backend
WeSocket is a common way to communication between a client and a server. It works exactly like TCP, but over HTTP. It has reliability and orderness, but is easier to implement in a web-browser.

The rust crate `ts-rs` provides a compilation from the Rust objects to typescript. In that way, we can use in the frontend the exact objects defined in the backend.

The messages are the followings :
- send the entier game state via the [`Infos`](../backend/src/objects/infos.rs) object
- send only the changes via the [`Event`](../backend/src/objects/infos_change.rs) object

## Docker
Docker allows this app to be run on any devices. I chose to put the front and backend in the same docker, to be able to run it with a really simple command : `docker run app`. It runs the front with nginx, and the back with a binary. The image is small : 110MB.

The build steps are defined in the [`Dockerfile`](../Dockerfile). To avoid CORS errors, nginx is used to link the front and backend on the same url and port.

## LoadTesting
The goal of the loadtesting is to run as many users as we can until the server slows down. To do that, the code connects in websocket to the backend, sends a user position. Then, it sends periodicly a movement to stay alive. Also, it returns the average ping.

The test continues until the server cannot accept more user, the ping is too high, or the time spent is greater than 1 min. When this condition is met, the test continues for 5 seconds with the same number of players.

# Dependencies
- webassembly : run the rust code in the browser and interact with the page
- websocket : communicate between the front and back
- serde : transforms rust object instances to json

# Sprints
1. basic functionnalities
    1. select username
    1. the player can move on a grid
    2. the player can eat an apple and grow
    3. on the walls, loop back
2. death
    1. the player loose if it hits any snake (himself or another player)
    2. the player loose if it hits a wall
3. loosing / winning screen
    1. if a player loose, a popup shows
    2. at the end, a scoreboard is shown
4. advance player movement
    1. handle rollback

# Design

## Idea
The idea is to create a snake game, but multiplayer. The idea is to copy the classical game :
- a fixed size grid
- only one apple for everyone
- movement every time $t$
- every apple increment the size of the player

## Architecture
The architecture is frontend - backend. The project can be run with a `docker-compose` file that runs everything. The pipeline allows the build and test of the project at every commit, and deploy the main branch.

The entire project will be written in **Rust**. 
The idea is to create a performant web-app, that can handle a large amount of users. It will be usefull espacially in the server-side.

The power of using the same language, means that we can share the objects between the fontend and backend.

```mermaid
---
title: Interactions
---
flowchart TB
Frontend -. Direction .-> Backend
Backend -. Player positions .-> Frontend
```

```mermaid
---
title: Architecture
---
flowchart TB

subgraph Docker
Nginx[[Nginx :80]] -- / --> Frontend
Nginx -- /websocket --> Backend

Frontend -.-> Objects
Backend -.-> Objects
end

Client --> Docker

```

### Backend
It will handle all the game logic. The position and the direction is stored here. It will receive the command from the users for the direction, and send the position of everyone and the apple.

### Frontend
The frontend will draw the players sent by the server. It will also send every keys sent by the user.

## Dependencies
- webassembly : communicate between the front and back

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

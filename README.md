ESIR3 SR Project - Multiplayer Snake Game
===

A multiplayer snake game.

# Members
- [Fabien GOARDOU](https://fabiengoardou.fr/)

# Design
See the [design file](./design/design.md)

# Demo
You can access the game at https://sr.fgdou.ovh/

# Docker
## Run it on your server
Just run :
```sh
docker run -p 80:80 ghcr.io/fgdou/sr_project:master
```
And access the website on http://localhost

## Build the project and Run
```sh
docker compose up --build
```

# Development
- [frontend](./frontend/)
- [backend](./backend/)

# Documentation
See the [design folder](./design/)
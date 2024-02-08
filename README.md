ESIR3 SR Project - Multiplayer Snake Game
===

![logo](./frontend/src/img/logo.png)

A multiplayer snake game.

# Demo
You can access the game at https://sr.fgdou.ovh/

# Members
- [Fabien GOARDOU](https://fabiengoardou.fr/)

# Documents
- [Design File](./doc/design.md)
- [Report](./doc/report.md)

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

# Load testing
You can run the load testing [here](./loadtesting/)

# Development
- [frontend](./frontend/)
- [backend](./backend/)

# Documentation
See the [design folder](./design/)
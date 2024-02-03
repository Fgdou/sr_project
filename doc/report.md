Report - Fabien GOARDOU
===

# Challenges

## Ping reliability
The `Client` classcalculate the average time difference that the network takes. With  this estimation, it can smooth out the receiving of packet, so that the game seems smoother. You can see the implementation in the [`MessageTPSSmoother`](../frontend/src/MessageTPSSmoother.ts) class.

## Error detection
For every message received, an id is linked to it. The `Client` module compares it, and if there is an error, ask the server to send the full information.

## XSS Attacks
When I first tried my game with my friends, they started putting html elements in the username... So it caused issues. So I implemented 2 conditions :
- must be 10 characters max
- must be only alphanumeric

# Work Organization
This project is organized aroung one tool : **GitHub**. It has a `master` and `develop` branches, along with a pipeline :
1. each commit to `develop` is build and tested
2. each merge to `master` has topass the build and will be published as a docker image

Also, to reference new features or bugs, I use the [GitHub issues](https://github.com/Fgdou/sr_project/issues) of the project. This way, I only focus on one main task at a time.
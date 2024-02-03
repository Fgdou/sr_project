Report - Fabien GOARDOU
===

# Challenges

## Ping reliability
The `Client` classcalculate the average time difference that the network takes. With  this estimation, it can smooth out the receiving of packet, so that the game seems smoother. You can see the implementation in the [`MessageTPSSmoother`](../frontend/src/MessageTPSSmoother.ts) class.

$$T_{wait}(n) = T_{n} - T_{n-1} - T_{frame}$$
$$T_{frame} = 300ms$$

For a long time, the ping was rughtly 100ms. But it turned out that [this commit](https://github.com/Fgdou/sr_project/commit/e6f07d262497ed2079f0debe3b342bcab02b4b32) fixed it to bring it down to 30ms. The issue was that I was storing only the high ping, which added up the error over time, instead of taking the average.

## Error detection
For every message received, an id is linked to it. The `Client` module compares it, and if there is an error, ask the server to send the full information.

## XSS Attacks
When I first tried my game with my friends, they started putting html elements in the username... So it caused issues. So I implemented 2 conditions :
1. must be 10 characters max
2. must be only alphanumeric

# Work Organization
This project is organized aroung one tool : **GitHub**. It has a `master` and `develop` branches, along with a pipeline :
1. each commit to `develop` is build and tested
2. each merge to `master` has topass the build and will be published as a docker image

Also, to reference new features or bugs, I use the [GitHub issues](https://github.com/Fgdou/sr_project/issues) of the project. This way, I only focus on one main task at a time.
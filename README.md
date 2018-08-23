# Microservice Comparison
Build a simple microservice in multiple languages to have a solid baseline to compare languages.

# What does it do?
1. Download json data from reddit
2. Transform it
3. Return transformed data

# Thoughts on implementations

## Rust
Batteries are not included. Have to go out and decide which async library // http server is right for your project. To do this basic workflow brought in 150+ transitive dependencies. Many of the code examples out there are using `use xxx::*;` which makes it hard to understand where things are coming from. Avoid use while you are learning.

Originally tried to go with [hyper http client](https://github.com/hyperium/hyper) until I realized hyper's http examples dont support https. Decided to go with [reqwest](https://github.com/seanmonstar/reqwest) but the current version of [reqwest's docs are broken](https://github.com/seanmonstar/reqwest/issues/323).

Rust requires a much larger investment in time to get moving. Larger being defined as a hour or two of reading about memory management for a senior engineer. Errors are normally helpful at suggesting next steps.

## Golang
Implementation-wise was pretty straight forward. Generic JSON access was really ugly and involved a lot of type casting. Went with a defined JSON type to clean up the code.

Go has a standard http server but routing is third party. Went with [echo](https://echo.labstack.com/) because the documentation and error handling was nice. Used std JSON support but its [known to be slow](https://github.com/golang/go/issues/5683). Consider picking a third party library if your payloads are large.

Implementation brought in 11 transitive dependencies. 

Reddit seems to be blacklisting go's http agent. Consistent 429s until I changed it.

## Crystal
Smallest implementation. Code is very ruby like. I appreciate the JSON builder concept. Release build can spike in time to 10s. [Build time horror stories on reddit](https://www.reddit.com/r/crystal_programming/comments/98s10f/will_crystal_survive/)make me nervous about using it for anything serious. But I doubt a microservice would bump into the multi minute long compile times.

Similar situation to golang in that http server built in router is third party. Implementation brought in 4 transitive dependencies. 
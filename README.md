# lisp_rs

An exploratory learning project! I'm building a simple Lisp interpreter (aiming for something Scheme-like) in Rust. It's far from full-featured but it has many basic features such as first-class functions, definitions, lexically-scoped closures, basic list operations (cons/car/cdr), control flow with `if`, a very simple numeric type, and booleans.

Next on the TODO list, whenever I get around to it: macros and tail call optimization!

To run: clone repo and `cargo run` in the directory. Probably helps to have cargo and rust installed!

June 2023:
- Created a web frontend using WebAssembly and Xterm.js. It was hard to find a simple way to glue them together but a crude version is deployed here: https://evestroud.github.io/lisp_rs/ It works but some basic readline functionality is missing (arrow keys don't work)

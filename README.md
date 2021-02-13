# Full stack Rust Demo

This project demonstrates how to glue a rocket app and a WASM frontend using
wasm-run.

Goals:

- Rocket backend + WASM frontend + SCSS styling
- No Javascript/NodeJS needed to build
- Handle debug/release modes
- Hot-compile for SCSS and WASM (but not hot reload in the browser yet)

## Logs

Just keep an eye on what I've tried and played with.

    $ cargo new fullstack
    $ cd fullstack

    # Setup cargo workspace with three crate: `backend`, `frontend`, `run`
    
    # No arguments passed:
    $ cargo run --bin run
    # does not start. It will loop forever displaying ``
    # `Finished dev [unoptimized + debuginfo] target(s) in 0.03s`
    # until the inotify limit is reached.


Q: What are the 



## Ideas


- I can't find in the doc the order of execution for stuff. I see the doc for
  the main macro. Are the named arguments that I pass to the macro 'hooks'?
  What is the order of execution? 
- The `serve` feature is useful when you only have a wasm app and no way to
  serve it, correct? If I have my backend app that will serve the wasm itself, I
  don't need it.
- Why is the backend-and-frontend example using a docker container? Is this a
  requirement? It seems that there are other_cli_commands available made for it
  that builds the docker container. How do I simply execute the app on my
  desktop without building the container? Isn't this example trying to
  demonstrate two separates things?
- What is the single `example.rs` file in the examples folder? Is it a
  standalone example?

- Logging in wasm-run could be helpful to debug the steps. 
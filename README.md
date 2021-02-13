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

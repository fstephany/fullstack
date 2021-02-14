# Full stack Rust Demo

This project demonstrates how to glue a rocket app and a WASM frontend using
wasm-run.

Goals:

- Rocket backend + WASM frontend + SCSS styling
- One command to start the whole shebang in development + display logs in console
- No Javascript/NodeJS needed to build
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
    # Shouldn't we return an error if no arguments are passed? Or what is 
    # supposed to do when no commands are given?


Q: What are the available CLI commands by default? `serve` and `build`? The
`--help` output is a bit sparsed.

Q: I see that I can add new custom commands? How does that look like?


Q: In the case of a pure WASM project, `build` and `serve` makes sense on their
own. When there other components, like this example with a backend, SCSS and
frontend, what do they mean? Is `build` for the three components? How should I
express that I only want to rebuild the backend (e.g., if a file changes for
example?).

Q: There's no mention in the README of the watcher. In the main macro
documentation we see 
> a function that is called when the watcher is being
> initialized (allowing you to add extra things to watch for example)

How is this working? Does wasm-run always initialize a watcher? When?

## Ideas


- I can't find in the doc the order of execution for stuff. I see the doc for
  the main macro. Are the named arguments that I pass to the macro 'hooks'?
  What is the order of execution? 
- The `serve` feature is useful when you only have a wasm app and no way to
  serve it, correct? If I have my backend app that will serve the wasm itself, I
  don't need it.
- Emphasize the two commands at the heart: `serve` and `build`.
- Why is the backend-and-frontend example using a docker container? Is this a
  requirement? It seems that there are other_cli_commands available made for it
  that builds the docker container. How do I simply execute the app on my
  desktop without building the container? Isn't this example trying to
  demonstrate two separates things?
- What is the single `example.rs` file in the examples folder? Is it a
  standalone example?

- Terminal output to see what is going on (e.g., a build is triggered after a
  change has been seen by the watcher)

  
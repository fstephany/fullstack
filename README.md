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


Q: I can't find in the doc the order of execution for stuff. I see the doc for
  the main macro. Are the named arguments that I pass to the macro 'hooks'?
  What is the order of execution? 


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

How is this working? Does wasm-run always initialize a watcher? What does it
watch?


Q: How does sass/scss building works? It seems to be part of the post_build hook
step. Will the sass/scss still be transpiled if I provide a post_build hook? Why
isn't this addivitive? 

Q: How do I prevent the generation of the index.html without overriding the
post_build hook? I don't need it as HTML is generated by my backend server.

Q: Oh! its the `post_build` step that writes the wasm+js to disk.

Q: Is the anyhow requirement appropriate? I guess we can afford it as the scope
is quite reduced?

Q: Oh ok, in the backend example in wasm-run, the binary (let's call it `main`)
is dependent on the backend and runs it from there. This means
* the backend must be startable from outside its main (what about dotenv for example?)
* it can be started without using an external process. As *it is* the running   process.
* 

## Ideas

- I feel like hooks should be different than steps. A hook should not overwrite
  a behaviour but be empty by default and only called if provided. Or is it just
  a naming issue? This point begs the question of balance between built-in
  feature and configurability. It seems that we're missing something here.

- Terminal output to see what is going on (e.g., a build is triggered after a
  change has been seen by the watcher)

- The heart of wasm-run is building and packaging rust wasm. Do we event want to
  extend it to cover this use case? 

- Shouldn't we separate the build of each components (backend, frontend, css)?
  Do we need to support other use case with more components? Like two separate
  frontend (eg., one for the public version of the app and another for an admin
  side)?

## Minor nitpicks:

- Doc could emphasize more the two commands at the heart: `serve` and `build`.
- What is the single `example.rs` file in the examples folder? Is it a
  standalone example?
- Why is the backend-and-frontend example using a docker container? Is this a
  requirement? It seems that there are other_cli_commands available made for it
  that builds the docker container. How do I simply execute the app on my
  desktop without building the container? Isn't this example trying to
  demonstrate two separates things?
- We could mention the alternative in the README (trunk, cargo-make). Or even
  maybe the whole WASM pieces and how the ecosystem works (websys, bindgen)
  
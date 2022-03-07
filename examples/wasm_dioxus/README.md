# Rollbar SDK <> Dioxus

This example shows using this SDK in a WASM context with the [Dioxus framework.](https://github.com/DioxusLabs/dioxus)

## Getting started

Install these prerequisites:

- [Rust toolkit.](https://rustup.rs/)
- [Trunk bundler.](https://trunkrs.dev/)

Then run the example using the following command from this directory:

```shell
trunk serve
```

Then open [localhost:8080](http://localhost:8080) and press the "send message"
button to send a message to Rollbar. You should now be able to see the message
at `https://rollbar.com/<you>/<your project>/items/`.

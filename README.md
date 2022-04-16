This is an umbrella project which contains a few different Rust crates for interacting with Rollbar.
Everything is still heavily under development and everything include crate names are subject to
change.

## Crates

### rollbar-rust

The subdirectory `crates/core` contains a crate which defines the types necessary to build items for the
Rollbar API, basic configuration, and an HTTP transport for sending items to the API. The intended
purpose of this crate is to serve as the foundation for interacting with the API for other things to
build upon.

I might change the name of this crate to something else and then have a higher level crate take over
the name of `rollbar-rust` which will be the Rust SDK. As of now this crate is not really a full
fledged SDK.

### rollbar-jvm

The subdirectory `crates/jvm_core` contains a crate which encapsulates certain interactions with the JVM
and JVMTI. Building this crate requires `JAVA_HOME` be set correctly so that we can get access to
the JVMTI C headers to generate Rust bindings. You can see what is necessary in the `build.rs` file
within this crate. This crate relies on `rollbar-rust` for some type definitions of Exceptions and
Frames. These are used for getting stack traces from the JNI/JVMTI.

### rollbar-java-agent

The subdirectory `crates/jvm_sdk_agent` contains a crate which builds a cdylib to be used as a native agent
with the JVM. This agent assumes existence and proper configuration of the `rollbar-java` SDK in
your Java project. It is intended to supplement the normal Java SDK by providing extra information
that is only possible to gather from the JVMTI. Currently this is only used for gathering local
variables. This crate is pretty small as most of the functionality is provided by `rollbar-jvm`.

### rollbar-jvm-agent

The subdirectory `crates/jvm_bare_agent` contains a crate which builds a cdylib to be used as a native
agent with the JVM. This agents assumes that `rollbar-java` does NOT exist in your Java project and
therefore does some of the work that would otherwise be done by the Java SDK. Again because
`rollbar-jvm` handles a lot of the heavy lifting of the interaction with the JVM this crate does not
have to do that much.

If you think the naming scheme is terrible I agree with you. Therefore, all of the actual names are
probably going to change once all of the parts are fleshed out. That is why nothing has been
published to crates.io yet.

### rollbar-wasm

The subdirectory `crates/wasm` contains a crate which can be built by
`wasm-pack` and then ran in the browser context. `examples/nextjs` contains an
example web application that consumes a package generated from this crate. To
run the example, ensure you have a file in the root of this repository named
`.env` that contains something like:

```
POST_TOKEN=<your post client token>
```

Next, ensure you have gnu make installed and run `make nextjs-example`.

### rollbar-wasm

The subdirectory `crates/node` contains a crate which which generates a binary
library that can be consumed as a native addon for node. The directory also
contains a typescript file and a `package.json` that complete an node module
that can be consumed by a node application. The `examples/nodejs` directory
contain an example of using the library. To run the example, ensure you have a
file in the root of this repository named `.env` that contains something like:

```
POST_TOKEN=<your post client token>
```

Next, ensure you have gnu make installed and run `make nextjs-example`.

## Building

* Install Rust: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
* `cargo build --release`
* Get what you want from `target/release`:
  * Agent to help Java SDK: `target/release/librollbar_java_agent.{so,dll,dylib}`
  * Agent without Java SDK: `target/release/librollbar_jvm_agent.{so,dll,dylib}`
* There is also a Dockerfile for building Linux releases on the Mac

## Building on a Mac for Linux

In the particular case where you are using a Mac but want to build a shared library that works on Linux, you have to do a little bit of extra work. Luckily, Rust has a decent cross compilation story. The first step is adding the right target via `rustup`:

* `rustup target add x86_64-unknown-linux-gnu`

This is not enough because you need a cross compiling toolchain, in particular a linker, that does the right thing. You can get this via:

* `brew tap SergioBenitez/osxct`
* `brew install x86_64-unknown-linux-gnu`
  - You might have to run `xcode-select --install` first depending on your setup

Once that is setup, you can build for the specified target:

* `cargo build --release --target x86_64-unknown-linux-gnu`

You will find the resulting `.so` located at:

```
target/x86_64-unknown-linux-gnu/release/librollbar_java_agent.so
```

Alternatively, you can use the `build-lib.sh` script which uses the Dockerfile in the root to build
a `.so` inside a Docker container and then copies it to your file system.

## Debugging

If you want to see additional output from our agent, you can set the environment variable
`ROLLBAR_LOG` to one of `trace`, `debug`, `info`, or `warn`. These will output different levels of
information to standard out where your JVM process is running.

## Using the agent

How to use the agent depends on how you invoke the JVM to start your application. In order
to use a native agent you need to pass a command line argument to this invocation. The most
basic usage would look like:

```
java -jar foo.jar -agentpath:path/to/librollbar_java_agent.dylib
```

However, if you are using a toolchain, such as Gradle, to manage your application then
adding this command line argument might take a bit more effort to figure out where to add it. For
Gradle the easiest way is to add the following to your `build.gradle` file:

```
applicationDefaultJvmArgs = ["-agentpath:path/to/"+System.mapLibraryName("rollbar_java_agent")]
```

Regardless of your JVM language of choice, at some level their is an invocation of the JVM and
therefore there is a configuration option to pass arguments directly to the JVM.

## Configuration

The agent that works with the Rollbar Java SDK does not need any configuration because it relies on
the Java SDK to be properly configured and to send items to the API. The other agent does need to be
configured. The agent assumes a file named `rollbar.conf` which lives at the same path as the
dynamic library. This file should be in TOML format, for example:

```
access_token = "abc123"
endpoint = "https://api.rollbar.com/api/1/item/"
timeout = 10
```

We are still working on the configuration, but the only required field is the `access_token`.

Instead of using a file, if the only thing you want to set is the `access_token`, you can also set
the `ROLLBAR_TOKEN` environment variable in your process with the value of your access token. If
`rollbar.conf` is present it will take precedence over the environment variable.

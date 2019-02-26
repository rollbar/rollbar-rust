#!/bin/bash
#
# Usage: ./build-lib <library_name>
# For example if we run:
#   ./build-lib rollbar_jvm_agent
# We will end up with librollbar_jvm_agent.so in the current directory

docker build -t build-"$1"-image .
docker run -it --name build-"$1" build-"$1"-image
docker cp build-"$1":/home/rust/src/target/x86_64-unknown-linux-gnu/release/lib"$1".so lib"$1".so
docker rm build-"$1"
docker rmi build-"$1"-image

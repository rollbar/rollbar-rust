.PHONY: build-image run-image clean-container clean-image all
.PHONY: build-centos run-centos centos-shell

ifneq (,$(wildcard ./.env))
    include .env
    export
endif

build-image:
	docker build -t build-rollbar_jvm_agent-image:latest .

build-centos:
	docker build -f Dockerfile.centos -t build-rollbar_jvm_agent-image-centos:latest .

run-image:
	docker run -it --rm --name build-rollbar_jvm_agent -v $(CURDIR)/target:/home/rust/src/target build-rollbar_jvm_agent-image

run-centos:
	docker run -it --rm --name build-rollbar_jvm_agent-centos -v $(CURDIR)/target:/target build-rollbar_jvm_agent-image-centos

clean-container:
	docker rm build-rollbar_jvm_agent

clean-image:
	docker rmi build-rollbar_jvm_agent-image

centos-shell:
	docker run -it build-rollbar_jvm_agent-image-centos:latest /bin/sh

all: build-image run-image

build-wasm:
	wasm-pack build --out-dir ../../build/wasm crates/wasm

create-wasm-link: build-wasm
	yarn --cwd ./build/wasm link

link-wasm: create-wasm-link
	yarn --cwd ./examples/nextjs link rollbar-wasm

before-nextjs-example: link-wasm
	yarn --cwd ./examples/nextjs install

nextjs-example: before-nextjs-example
	yarn --cwd ./examples/nextjs dev

before-build-node:
	yarn --cwd ./crates/node install

build-node: before-build-node
	yarn --cwd ./crates/node build

create-node-link: build-node
	yarn --cwd ./crates/node link

link-node: create-node-link
	yarn --cwd ./examples/nodejs link rollbar-node

before-nodejs-example: link-node
	yarn --cwd ./examples/nodejs install

nodejs-example: before-nodejs-example
	node ./examples/nodejs/index.js

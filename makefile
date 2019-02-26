.PHONY: build-image run-image clean-container clean-image all

build-image:
	docker build -t build-rollbar_jvm_agent-image:latest .

run-image:
	docker run -it --rm --name build-rollbar_jvm_agent -v $(CURDIR)/target:/home/rust/src/target build-rollbar_jvm_agent-image

clean-container:
	docker rm build-rollbar_jvm_agent

clean-image:
	docker rmi build-rollbar_jvm_agent-image

all: build-image run-image

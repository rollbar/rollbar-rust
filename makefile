.PHONY: build-image run-image clean-container clean-image all
.PHONY: build-centos run-centos centos-shell

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

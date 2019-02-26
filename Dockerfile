FROM ekidd/rust-musl-builder

RUN sudo apt-get update && \
    sudo apt-get install -y \
      openjdk-8-jdk \
      default-jre \
      clang

ENV JAVA_HOME /usr/lib/jvm/java-8-openjdk-amd64/

ADD . ./

CMD cargo build --release --target x86_64-unknown-linux-gnu

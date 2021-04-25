FROM ubuntu


# install target toolchain
RUN apt-get update --fix-missing
RUN apt-get install -y build-essential gcc-arm-linux-gnueabihf

# general dependencies
RUN apt-get install -y curl

# install Rust
WORKDIR /root
RUN curl https://sh.rustup.rs -sSf > rustup.sh
RUN chmod +x rustup.sh
RUN /bin/sh -c './rustup.sh -y'
RUN echo 'export PATH="$HOME/.cargo/bin:$PATH"' > /root/.bashrc

# install cross-compiled standard crates
RUN /root/.cargo/bin/rustup target add armv7-unknown-linux-gnueabihf

# configure cargo for cross compilation
COPY ./configure.sh /root/configure.sh
RUN chmod +x /root/configure.sh
RUN /root/configure.sh

# cache the crates that are in use
COPY ./Cargo.toml /root/workspace/Cargo.toml
COPY ./Cargo.lock /root/workspace/Cargo.lock
WORKDIR /root/workspace
ENV USER root
ENV PATH $PATH:/root/.cargo/bin
RUN cargo update

CMD cargo build --release --target=armv7-unknown-linux-gnueabihf
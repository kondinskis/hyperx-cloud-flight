FROM debian:bookworm

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && \
    apt-get install -y devscripts equivs curl

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
		bash -s -- -y

COPY . /build
WORKDIR /build

RUN bash -c "source ~/.bashrc && mkdir .cargo && cargo vendor >>.cargo/config.toml"
RUN mk-build-deps -i -t 'apt-get -y --no-install-recommends' && \
    dpkg-buildpackage -us -uc -b

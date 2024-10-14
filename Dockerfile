# compile
FROM	alpine:3.14 AS compiler

RUN	apk update --quiet
RUN	apk add curl openssl-dev
RUN	apk add build-base

RUN	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN	echo '[source.crates-io]' > $HOME/.cargo/config
RUN	echo 'replace-with = "rsproxy"' >> $HOME/.cargo/config
RUN	echo '[source.rsproxy]' >> $HOME/.cargo/config
RUN	echo 'registry = "https://rsproxy.cn/crates.io-index"' >> $HOME/.cargo/config

ENV	RUSTFLAGS="-C target-feature=-crt-static"

WORKDIR	/yairmov

COPY	. .
RUN	$HOME/.cargo/bin/cargo build --release

# Run
FROM	alpine:3.14

RUN	apk add -q --no-cache libgcc tzdata
RUN	cp /usr/share/zoneinfo/Asia/Shanghai /etc/localtime
RUN	echo "Asia/Shanghai" > /etc/timezone
RUN	apk del tzdata

COPY	--from=compiler /yairmov/target/release/app .
COPY	--from=compiler /yairmov/.env .

ENV	BIND_ADDRESS=0.0.0.0:5896

EXPOSE	5896/tcp

CMD	./app

FROM ubuntu:22.04
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    curl \
    gcc libc6-dev ca-certificates bzip2 \
    libssl-dev pkg-config

ENV RUSTUP_INIT_SKIP_PATH_CHECK="yes"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y -v --no-modify-path
RUN cp ${HOME}/.cargo/bin/* /usr/local/bin/

RUN cargo install --git https://github.com/anza-xyz/cargo-run-solana-tests.git \
    --rev 2ff58e81919c5c6580620f4b1f0e37777ef6787f \
    --bin cargo-run-solana-tests --root /usr/local

RUN mkdir -p /tmp/.cache/solana/v1.50/platform-tools
RUN curl -L -o platform-tools-linux-x86_64.tar.bz2 https://github.com/anza-xyz/platform-tools/releases/download/v1.50/platform-tools-linux-x86_64.tar.bz2
RUN tar -xjf platform-tools-linux-x86_64.tar.bz2 --strip-components 1 -C /tmp/.cache/solana/v1.50/platform-tools
RUN rustup toolchain link solana /tmp/.cache/solana/v1.50/platform-tools/rust
RUN cp -R ${HOME}/.rustup /tmp/

ENV CARGO_TARGET_SBPF_SOLANA_SOLANA_RUNNER="cargo-run-solana-tests --heap-size 104857600"
ENV LLVM_HOME="/tmp/.cache/solana/v1.50/platform-tools/llvm"
ENV CC="/tmp/.cache/solana/v1.50/platform-tools/llvm/bin/clang"
ENV RUSTUP_TOOLCHAIN="solana"

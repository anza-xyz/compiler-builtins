FROM ubuntu:20.04
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    curl \
    gcc libc6-dev ca-certificates

ENV RUSTUP_INIT_SKIP_PATH_CHECK="yes"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y -v --no-modify-path
RUN cp ${HOME}/.cargo/bin/* /usr/local/bin/

RUN cargo install --git https://github.com/solana-labs/cargo-run-solana-tests.git \
    --rev df2f642924aee7bbd2566017b3d71cb0c389b015 \
    --bin cargo-run-solana-tests --root /usr/local

RUN mkdir -p /tmp/.cache/solana/v1.38/platform-tools
RUN curl -L -o platform-tools-linux-x86_64.tar.bz2 https://github.com/solana-labs/platform-tools/releases/download/v1.38/platform-tools-linux-x86_64.tar.bz2
RUN tar -xjf platform-tools-linux-x86_64.tar.bz2 --strip-components 1 -C /tmp/.cache/solana/v1.38/platform-tools
RUN rustup toolchain link solana /tmp/.cache/solana/v1.38/platform-tools/rust
RUN cp -R ${HOME}/.rustup /tmp/

ENV CARGO_TARGET_SBF_SOLANA_SOLANA_RUNNER="cargo-run-solana-tests --heap-size 104857600"
ENV CC="/tmp/.cache/solana/v1.38/platform-tools/llvm/bin/clang"
ENV RUSTUP_TOOLCHAIN="solana"

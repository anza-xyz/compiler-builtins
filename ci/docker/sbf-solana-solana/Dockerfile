FROM ubuntu:20.04
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    curl \
    gcc libc6-dev ca-certificates

ENV RUSTUP_INIT_SKIP_PATH_CHECK="yes"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y -v --no-modify-path
RUN cp ${HOME}/.cargo/bin/* /usr/local/bin/

RUN cargo install --git https://github.com/solana-labs/cargo-run-bpf-tests.git \
    --rev c50617adeb8a72d3366e02dd3e37e464fdaca1df \
    --bin cargo-run-bpf-tests --root /usr/local

RUN mkdir -p /tmp/.cache/solana/v1.22/bpf-tools
RUN curl -L -o solana-sbf-tools-linux.tar.bz2 https://github.com/solana-labs/bpf-tools/releases/download/v1.22/solana-sbf-tools-linux.tar.bz2
RUN tar -xjf solana-sbf-tools-linux.tar.bz2 --strip-components 1 -C /tmp/.cache/solana/v1.22/bpf-tools
RUN rustup toolchain link bpf /tmp/.cache/solana/v1.22/bpf-tools/rust
RUN cp -R ${HOME}/.rustup /tmp/

ENV CARGO_TARGET_SBF_SOLANA_SOLANA_RUNNER="cargo-run-bpf-tests --heap-size 104857600"

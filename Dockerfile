# Use official Debian-based Node image (Alpine causes musl build issues with Anchor)
FROM node:22-bullseye

# Install OS dependencies
RUN apt-get update && apt-get install -y \
  curl git pkg-config build-essential libssl-dev libudev-dev python3 python3-pip \
  && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"

# Install Anchor CLI
RUN cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked

# Install Solana CLI
RUN curl -sSfL https://release.solana.com/stable/install | bash
ENV PATH="/root/.local/share/solana/install/active_release/bin:$PATH"

# Set working directory and copy code
WORKDIR /app
COPY . .

# Build Anchor program
RUN anchor build

# Default CMD for Render (no startCommand allowed)
CMD solana config set --url "$SOLANA_RPC" && anchor deploy

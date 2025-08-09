# ---------- Builder: compile and generate ----------
FROM rust:1.82-bookworm AS builder

WORKDIR /app

# Pre-copy manifest for better layer caching (no external deps used, but keeps structure standard)
COPY Cargo.toml Cargo.lock ./
# Prefetch dependencies (lockfile respected) without requiring real sources
RUN mkdir -p src && echo 'fn main(){}' > src/main.rs && cargo fetch --locked

# Copy source
COPY src ./src
COPY Makefile ./Makefile
COPY README.md ./README.md

# Provide test fixtures needed by unit tests without copying full sites
RUN mkdir -p sites
COPY sites/test ./sites/test

# Run tests in single-threaded mode per repo rules
RUN cargo test -- --test-threads=1

# Build release binary
RUN cargo build --release

# Copy sites only after the binary is built for better caching
COPY sites ./sites

# Generate sites (outputs into ./out/<site_name>)
RUN ./target/release/lepkefing generate polgarhivatal.nl \
 && ./target/release/lepkefing generate lepkef.ing


# ---------- Runtime: nginx to serve generated output ----------
FROM nginx:alpine AS runtime

# Replace default server config with two vhosts
COPY nginx/default.conf /etc/nginx/conf.d/default.conf

# Copy generated static sites
COPY --from=builder /app/out /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"] 
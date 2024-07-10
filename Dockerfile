FROM rust:latest AS chef

WORKDIR /render_cdk

RUN cargo install cargo-chef --locked

FROM chef AS planner

COPY ./rust .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS bulder

COPY --from=planner /render_cdk/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY ./rust .

RUN cargo build --release

# CMD [ "./target/release/render_cdk" ]
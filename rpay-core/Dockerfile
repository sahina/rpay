FROM rust:1.56 as builder

ENV CARGO_TERM_COLOR always
RUN USER=root cargo new --lib rpay
WORKDIR ./rpay
COPY . .

RUN cargo build --release \
    && rm src/*.rs target/release/deps/rpay*

ADD . ./

RUN cargo build --release

FROM debian:buster-slim

ARG APP=/usr/src/app
ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /rpay/target/release/rpay ${APP}/rpay
RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}
CMD ["./rpay"]
EXPOSE 3000
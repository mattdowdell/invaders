#
#
#

FROM rust:1.60-slim

RUN set-eux; \
    export DEBIAN_FRONTEND=noninteractive; \
    apt-get update; \
    apt-get install --assume-yes --no-install-recommends \
        make \
    ; \
    rm -rf /var/lib/apt/lists/*;

ARG USER=default
ARG GROUP=${USER}
ARG HOMEDIR=/build
ARG UID=1000
ARG GID=1000

RUN set -eux; \
    addgroup \
        --gid ${GID} \
        ${GROUP} \
    ; \
    adduser \
        --quiet \
        --home ${HOMEDIR} \
        --uid ${UID} \
        --gid ${GID} \
        --gecos '' \
        --disabled-password \
        ${USER} \
    ;

USER ${USER}
WORKDIR ${HOMEDIR}

COPY --chown=${USER}:${GROUP} ./Makefile   ./Makefile
COPY --chown=${USER}:${GROUP} ./Cargo.toml ./Cargo.toml
COPY --chown=${USER}:${GROUP} ./Cargo.lock ./Cargo.lock
COPY --chown=${USER}:${GROUP} ./src        ./src

RUN set -eux; \
    make build;

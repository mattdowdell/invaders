#
#
#

CARGO  := cargo
DOCKER := docker
TAR    := tar

TARGET = $(shell rustc -Vv | grep host | awk '{ print $$2 }')

DOCKER_IMAGE := invaders-build
DOCKER_TAG   := local
DOCKER_NAME  := invaders-build

## help: Output this message and exit.
help:
	@echo invaders
	@echo ========
	@echo
	@fgrep -h '##' $(MAKEFILE_LIST) | fgrep -v fgrep | column -t -s ':' | sed -e 's/## //'
.PHONY: help

## all: Run all build targets.
all: build build-docker
.PHONY: all

## build: Create release build for the current target.
build:
	$(CARGO) build --release --target $(TARGET)
	$(TAR) -C ./target/$(TARGET)/release/ -cvzf invaders-$(TARGET).tar.gz invaders
.PHONY: build

## build-docker: Create release build for x86_64-unknown-linux-gnu using Docker.
build-docker:
	$(DOCKER) build --pull --force-rm -t $(DOCKER_IMAGE) .
	$(DOCKER) create --name $(DOCKER_NAME) $(DOCKER_IMAGE) ls
	$(DOCKER) start $(DOCKER_NAME)
	$(DOCKER) cp $(DOCKER_NAME):/build/invaders-x86_64-unknown-linux-gnu.tar.gz ./
	$(DOCKER) stop $(DOCKER_NAME)
	$(DOCKER) rm $(DOCKER_NAME)
.PHONY: build-docker

## clean: Delete build artifacts.
clean:
	rm -f invaders-*.tar.gz
.PHONY: clean

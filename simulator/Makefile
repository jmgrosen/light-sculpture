# Copyright (c) 2013 John Grosen under the terms described
# in the LICENSE file at the top of this repository

RUSTC = rustc
RSFILES = $(shell find . -type f -name '*.rs')
RUSTFLAGS = -L rust-opengles/ -L glfw-rs/lib/ -O

all: lightsculpture

lightsculpture: simulator.rs $(RSFILES)
	$(RUSTC) $(RUSTFLAGS) $< -o $@

clean:
	rm -f lightsculpture
	rm -rf lightsculpture.dSYM

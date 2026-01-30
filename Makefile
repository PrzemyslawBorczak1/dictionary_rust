CRATE_NAME := dictionary
RUST_MODE  := release

RUST_TARGET_DIR := target/$(RUST_MODE)
RUST_SO         := $(RUST_TARGET_DIR)/lib$(CRATE_NAME).so

C_DIR := c_usage
C_MAIN     := $(C_DIR)/main.c
C_OUT      := $(C_DIR)/app

CC      := gcc
CFLAGS  := -I $(C_DIR) -O2 -Wall -Wextra
LDFLAGS := -L $(RUST_TARGET_DIR) -l$(CRATE_NAME) -Wl,-rpath,'$$ORIGIN/../$(RUST_TARGET_DIR)'

.PHONY: all rust c run clean

all: $(C_OUT)

rust: $(RUST_SO)

$(RUST_SO):
	cargo build --$(RUST_MODE)

c: $(C_OUT)

$(C_OUT): rust $(C_MAIN) $(C_DIR)/dict.h
	$(CC) $(CFLAGS) $(C_MAIN) $(LDFLAGS) -o $(C_OUT)

run: all
	./$(C_OUT)

clean:
	cargo clean
	rm -f $(C_OUT)

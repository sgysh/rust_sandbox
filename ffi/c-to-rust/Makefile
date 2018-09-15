LDFLAGS := -Wl,--gc-sections -lpthread -ldl

all: target/calc
	target/calc

target:
	mkdir -p $@

target/calc: target/main.o target/debug/libcalc.a
	$(CC) -o $@ $^ $(LDFLAGS)

target/debug/libcalc.a: src/lib.rs Cargo.toml
	cargo build

target/main.o: src/main.c | target
	$(CC) -o $@ -c $<

clean:
	rm -rf target

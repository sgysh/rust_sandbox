LDFLAGS := -Wl,--gc-sections -lpthread -ldl

all: target/calc
	target/calc

target:
	mkdir -p $@

target/calc: target/main.o target/debug/libcalc.a
	$(CXX) -o $@ $^ $(LDFLAGS)

target/debug/libcalc.a: src/lib.rs Cargo.toml
	cargo build

target/main.o: src/main.cc | target
	$(CXX) -o $@ -c $<

clean:
	rm -rf target

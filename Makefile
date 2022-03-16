run: gp_c.bin gp_rs_unsafe.bin gp_rs_safe.bin
	@echo "==== Running C example ===="
	LD_LIBRARY_PATH=gp_c/ ./gp_c.bin
	@echo
	@echo "==== Running RUST (Unsafe) example ===="
	LD_LIBRARY_PATH=gp_c/ ./gp_rs_unsafe.bin
	@echo
	@echo "==== Running RUST (Safe) example ===="
	LD_LIBRARY_PATH=gp_c/ ./gp_rs_safe.bin

gp_rs_unsafe.bin: gp_rs/gp_sys/gp_sys.rs gp_rs/gp/gp_unsafe.rs
	RUSTFLAGS="-Lgp_c" cargo build --bin gp_unsafe && cp target/debug/gp_unsafe gp_rs_unsafe.bin

gp_rs_safe.bin: gp_rs/gp_sys/gp_sys.rs gp_rs/gp_safe/gp_safe.rs gp_rs/gp/gp_safe.rs
	RUSTFLAGS="-Lgp_c" cargo build --bin gp_safe && cp target/debug/gp_safe gp_rs_safe.bin

gp_c.bin: gp_c/main.c gp_c/libgp.so
	cc -ggdb -O0 -o gp_c.bin -Igp_c -Lgp_c -lgp gp_c/main.c

gp_c/libgp.so: gp_c/gp.c
	cc -ggdb -O0 -fPIC -shared -o gp_c/libgp.so -Igp_c gp_c/gp.c

clean:
	rm -f gp_c/*.o gp_c/libgp.so gp_c.bin gp_rs_unsafe.bin gp_rs_safe.bin
	cargo clean

valgrind: gp_c/main
	LD_LIBRARY_PATH=gp_c/ valgrind ./gp_c/main

gdb: gp_c/main
	LD_LIBRARY_PATH=gp_c/ gdb ./gp_c/main

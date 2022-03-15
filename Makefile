run: gp_c.bin gp_rs.bin
	@echo "==== Running C example ===="
	LD_LIBRARY_PATH=gp_c/ ./gp_c.bin
	@echo
	@echo "==== Running RUST example ===="
	LD_LIBRARY_PATH=gp_c/ ./gp_rs.bin

gp_rs.bin: gp_rs/gp.rs gp_rs/main.rs
	rustc -g -Lgp_c -o gp_rs.bin -lgp gp_rs/main.rs

gp_c.bin: gp_c/main.c gp_c/libgp.so
	cc -ggdb -O0 -o gp_c.bin -Igp_c -Lgp_c -lgp gp_c/main.c

gp_c/libgp.so: gp_c/gp.c
	cc -ggdb -O0 -fPIC -shared -o gp_c/libgp.so -Igp_c gp_c/gp.c

clean:
	rm -f gp_c/*.o gp_c/libgp.so gp_c.bin gp_rs.bin

valgrind: gp_c/main
	LD_LIBRARY_PATH=gp_c/ valgrind ./gp_c/main

gdb: gp_c/main
	LD_LIBRARY_PATH=gp_c/ gdb ./gp_c/main

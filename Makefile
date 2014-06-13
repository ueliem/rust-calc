clean:
	rm -rf build/*
	rmdir build/
run:
	./build/main
.PHONY: build
build:
	mkdir -p build
	rustc src/main.rs --out-dir build/

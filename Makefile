build:
	mkdir -p build
	rustc src/main.rs --out-dir build/
clean:
	rm -rf build/*
	rmdir build/
run:
	./build/main

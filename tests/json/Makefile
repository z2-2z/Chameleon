
run: main.c json.c json.c.h
	gcc -Wall -Wextra -Wpedantic -g -o $@ -O3 -flto $^

json.c json.c.h: ../../grammars/json.chm
	cargo run --release -- -o json.c --allow-cycles $<

.PHONY: clean
clean:
	@rm -fv json.c json.c.h run

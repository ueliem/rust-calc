rust-calc
===========
This was an experiment in writing a recursive descent parser in Rust. It is also my first "complete" parser. This is written entirely by hand.

When run, it gives a prompt that takes in a string. The string is then passed through the tokenizer, and the tokens are parsed in the parser.

tokenize.rs contains the tokenizer code. It is a simple finite state machine that can distinguish between a small set of typical mathematical symbols for addition, subtraction, multiplication, division, and parentheses.

token.rs contains the Token struct output by the tokenizer.

mathparse.rs contains the recursive grammar for the simple math language.

main.rs is just the command line interface.

todo:
===========
1. Add error handling
2. Improve command line input
3. Allow for multiplication by side-by-side parentheses

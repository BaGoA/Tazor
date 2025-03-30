# Tazor
Tazor is Rust library implementing a calculator based on mathematical expression

## Build
Build of Tazor is made by [Rust](https://www.rust-lang.org/) tool [Cargo](https://doc.rust-lang.org/cargo/)

To build Tazor, you can use the following command:

	*cargo build* to compile in debug mode
	*cargo build --release* to compile in release mode

To launch Tazor units tests, you can use the following command:

	*cargo test* to launch tests in debug mode
	*cargo test --release* to launch tests in release mode

## Code Documentation
Tazor code documentation is made also by Cargo with the following command:

	*cargo doc* to generate the documentation
	*cargo doc --open* to open the documention in your browser

## Documentation
Tazor contains structure called _Calculator_ which can handle several kind of expression:
- raw expression as '1 + 1' or 'cos(pi) * sqrt(2)'
- expression defining a variable like this 'x = 1 + 1'
- expression defining a function like this 'f: x, y = x * x + y * y

This structure stores previously defined variables and functions to reuse them in another expression.


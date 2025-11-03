fn main() {
    println!("Hello, world!");
}

// > rustc main.rs --> This compiles the code into an executable named `main`
// > .\main  --> This runs the compiled executable and prints:
// Hello, world!

// Rust is a systems programming language focused on safety, speed, and concurrency.
// It achieves memory safety without a garbage collector, making it suitable for performance-critical applications.
// The `println!` macro is used to print text to the console.
// The `fn main()` function is the entry point of a Rust program.

//Rust code does compiles the souce code if there is any syntax error it will show the error in the terminal and will not generate the executable file.
// which helps to avoid syntax errors at the compilation time itself.

//IF we make any chanes in the code we need to recompile the code using rustc command to reflect the changes in the executable file.

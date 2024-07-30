# Base64_encoder

This is a simple command-line application written in Rust that encodes input data to Base64 format.

## What is Base64?

Base64 is a binary-to-text encoding scheme that represents binary data in an ASCII string format. It's commonly used to encode binary data that needs to be stored and transferred over media that are designed to deal with text. This encoding helps ensure that the data remains intact without modification during transport.

## What This Application Does

This Rust application:

1. Reads input data either from a file or standard input (stdin).
2. Encodes the input data to Base64 format.
3. Outputs the encoded data to standard output (stdout).
4. Formats the output in lines of 76 characters each, as per common Base64 encoding practices.

## How to Build

To build the application:

1. Ensure you have Rust and Cargo installed on your system.
2. Clone this repository or download the source code.
3. Navigate to the project directory in your terminal.
4. Run the following command: `cargo build --release`

The compiled binary will be located in `target/release/base64_converter`.

## How to Use

You can use this Base64 encoder in two ways:

### 1. Encoding a File

To encode the contents of a file:
`./target/release/base64_converter path/to/your/file`

Replace `path/to/your/file` with the actual path to the file you want to encode.

### 2. Encoding from Standard Input

To encode data from standard input:
`./target/release/base64_converter`

Then type or paste your input. Press Ctrl+D (Unix) or Ctrl+Z (Windows) to signal the end of input.

Alternatively, you can pipe data into the program:
`echo "Hello, World!" | ./target/release/base64_converter`

### Saving Output to a File

To save the encoded output to a file, use output redirection:
`./target/release/base64_converter input_file.txt > output.txt`

## Notes

- This encoder can handle any type of file, not just text files.
- The program will display a usage message if you provide more than one argument or an invalid file path.
- The output is formatted in lines of 76 characters each, following common Base64 encoding practices.

## Error Handling

The application uses Rust's robust error handling mechanisms. If any errors occur (e.g., file not found, read/write errors), the program will exit and display an error message.


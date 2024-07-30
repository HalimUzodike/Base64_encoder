// Import necessary modules from the standard library
use std::env;          // For accessing command-line arguments
use std::fs::File;     // For file operations
use std::io::{self, Read, Write};  // For input/output operations
use std::process;      // For process control (exit)

// Define the Base64 alphabet as a constant byte array
// This array contains all characters used in Base64 encoding
const B64_ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

// Main function that returns a Result
// The Ok variant is an empty tuple (), and the Err variant is io::Error
fn main() -> io::Result<()> {
    // Collect command-line arguments into a vector of Strings
    let args: Vec<String> = env::args().collect();

    // Check if there are too many arguments
    if args.len() > 2 {
        // Print usage message to stderr
        eprintln!("Usage: {} [FILE]", args[0]);
        // Exit the program with a non-zero status code
        process::exit(1);
    }

    // Determine the input source: file or stdin
    // Box<dyn Read> is a trait object that can hold any type implementing the Read trait
    let mut input: Box<dyn Read> = if args.len() == 2 && args[1] != "-" {
        // If a file is specified and it's not "-", open the file
        // The ? operator will return early if there's an error opening the file
        Box::new(File::open(&args[1])?)
    } else {
        // Otherwise, use standard input
        Box::new(io::stdin())
    };

    // Initialize a counter for the number of characters printed
    let mut printed_chars = 0;
    // Create a buffer to hold input bytes (3 bytes at a time)
    let mut buffer = [0u8; 3];

    // Start the main processing loop
    loop {
        // Read up to 3 bytes from the input
        // bytes_read will contain the number of bytes actually read
        let bytes_read = input.read(&mut buffer)?;
        // If no bytes were read, we've reached the end of the input
        if bytes_read == 0 {
            break;
        }

        // Initialize the output buffer with padding characters
        let mut output = [b'='; 4];
        // Calculate the Base64 alphabet indices for the input bytes
        let alphabet_indices = [
            buffer[0] >> 2,  // First 6 bits of first byte
            ((buffer[0] & 0b11) << 4) | (buffer[1] >> 4),  // Last 2 bits of first byte + first 4 bits of second byte
            ((buffer[1] & 0b1111) << 2) | (buffer[2] >> 6),  // Last 4 bits of second byte + first 2 bits of third byte
            buffer[2] & 0b111111,  // Last 6 bits of third byte
        ];

        // Convert the calculated indices to Base64 characters
        // We only process as many indices as we have input bytes (plus one for padding)
        for (i, &index) in alphabet_indices.iter().enumerate().take(bytes_read + 1) {
            output[i] = B64_ALPHABET[index as usize];
        }

        // Write the encoded output to stdout
        io::stdout().write_all(&output)?;
        // Increment the character count
        printed_chars += 4;

        // If we've printed 76 characters, print a newline
        // This is for formatting the output in 76-character lines
        if printed_chars >= 76 {
            println!();
            printed_chars = 0;
        }
    }

    // If we've printed any characters in the last line, print a final newline
    if printed_chars > 0 {
        println!();
    }

    // Return Ok to indicate successful execution
    Ok(())
}
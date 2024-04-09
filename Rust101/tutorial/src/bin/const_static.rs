const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

fn main() {
    let digest = compute_digest("Hello");
    println!("Digest: {digest:?}");
}

// This is a small program that demonstrates how to compute a digest of a string in Rust.

// DIGEST_SIZE is a constant variable that holds the size of the digest.
// ZERO is a constant variable that holds the value 42 wrapped in an Option<u8>.
// The compute_digest function takes a reference to a string, and returns an array of bytes of size DIGEST_SIZE.

// The function starts by creating a mutable variable digest which is an array of DIGEST_SIZE elements, each initialized to ZERO.unwrap_or(0). The unwrap_or method is used to get the value of ZERO if it is Some(42), otherwise it returns 0.

// Then it uses a for loop to iterate over the bytes of the input text. The loop variable idx holds the current index, and b holds the current byte. The loop uses the enumerate() method on the bytes of the text to get both the index and the byte value for each iteration.

// In the loop body, it updates the element at the current index modulo DIGEST_SIZE of the digest array by adding the current byte. This is done using the wrapping_add method which is a safe way to add two numbers and wrap the result if it exceeds the maximum value of the type.

// Finally, the function returns the digest.

// In the main function, it calls the compute_digest function passing in the string "Hello" and assigns the result to a variable named digest, then prints it using the println! macro.

// The :? in the format string is used to print the digest as an array of bytes in a debug format
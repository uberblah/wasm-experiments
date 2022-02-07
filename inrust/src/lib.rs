/**
 * This is how we declare imported methods in Rust's WASM unknown target.
 * 
 * Note that imported and exported methods must only accept and return values
 * which can be hidden behind an i32 representation.
 */
extern "C" {
    fn notifyDone(value: i32);
}

/**
 * We can no longer use the zero address as the start of our IO buffer,
 * because Rust needs that space for data. We will use this static buffer
 * as the IO buffer.
 */
#[no_mangle]
pub static mut XCHG_BUF: [u8; 2] = [0; 2];

/**
 * Add the two numbers, let the caller know about it,
 * and then grab the value they sent us in the IO buffer.
 * 
 * In a real application we might pass encoded data structures
 * in the IO buffer, both as input and output to our functions.
 */
#[no_mangle]
pub unsafe fn add_2_nums(a: i32, b: i32) -> i32 {
    notifyDone(a + b); // call a callback provided by the caller, using args they provided
    XCHG_BUF[1] = 0xff; // write a hardcoded value to memory for the caller to find
    (XCHG_BUF[0] as i32) + a + b // return a value to the caller based on args and a value they wrote to memory
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

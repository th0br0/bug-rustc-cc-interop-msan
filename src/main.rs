use msan_repro::{cpp_return_large, pass_padded, Padded};

fn main() {
    println!("Calling pass_padded(Padded::Small([1])) to poison __msan_param_tls[0]...");
    pass_padded(Padded::Small([1]));

    println!("Calling C++ sret function cpp_return_large(42)...");
    let large = unsafe { cpp_return_large(42) };
    println!("Result: {:?}", large.0);
}

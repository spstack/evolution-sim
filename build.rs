/**
 * Build script that augments the Cargo.toml to specify more advanced options (as far as I can tell)
 * 
 * This is really just used to link the required external C driver for the matrix LED panels
 */

fn main() {
    // Command to cargo to tell it to link the librgbmatrix library statically
    println!("cargo::rustc-link-lib=static=/usr/lib/librgbmatrix/librgbmatrix");
}
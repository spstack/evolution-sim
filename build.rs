/**
 * Build script that augments the Cargo.toml to specify more advanced options (as far as I can tell)
 * 
 * This is used to build/link the required external C driver for the matrix LED panels
 */


fn main() {
    // Command to cargo to tell it to link the librgbmatrix library statically
    // println!("cargo::rustc-link-search=/usr/lib/librgbmatrix");
    // println!("cargo::rustc-link-lib=static=rgbmatrix");
    cc::Build::new()
        .cpp(true)
        .includes(["src/rpi-rgb-led-matrix/include"])
        .file("src/rpi-rgb-led-matrix/lib/bdf-font.cc")
        .file("src/rpi-rgb-led-matrix/lib/content-streamer.cc")
        .file("src/rpi-rgb-led-matrix/lib/framebuffer.cc")
        .file("src/rpi-rgb-led-matrix/lib/gpio.cc")
        .file("src/rpi-rgb-led-matrix/lib/hardware-mapping.c")
        .file("src/rpi-rgb-led-matrix/lib/led-matrix.cc")
        .file("src/rpi-rgb-led-matrix/lib/pixel-mapper.cc")
        .file("src/rpi-rgb-led-matrix/lib/graphics.cc")
        .file("src/rpi-rgb-led-matrix/lib/led-matrix-c.cc")
        .file("src/rpi-rgb-led-matrix/lib/multiplex-mappers.cc")
        .file("src/rpi-rgb-led-matrix/lib/options-initialize.cc")
        .file("src/rpi-rgb-led-matrix/lib/thread.cc")
        .cpp_link_stdlib("stdc++")
        .compile("rgbmatrix");
}
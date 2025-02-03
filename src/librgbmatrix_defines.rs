/* automatically generated by rust-bindgen 0.71.1 */


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RGBLedMatrix {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LedCanvas {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LedFont {
    _unused: [u8; 0],
}
#[doc = " Parameters to create a new matrix.\n\n To get the defaults, non-set values have to be initialized to zero, so you\n should zero out this struct before setting anything."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RGBLedMatrixOptions {
    pub hardware_mapping: *const ::std::os::raw::c_char,
    pub rows: ::std::os::raw::c_int,
    pub cols: ::std::os::raw::c_int,
    pub chain_length: ::std::os::raw::c_int,
    pub parallel: ::std::os::raw::c_int,
    pub pwm_bits: ::std::os::raw::c_int,
    pub pwm_lsb_nanoseconds: ::std::os::raw::c_int,
    pub pwm_dither_bits: ::std::os::raw::c_int,
    pub brightness: ::std::os::raw::c_int,
    pub scan_mode: ::std::os::raw::c_int,
    pub row_address_type: ::std::os::raw::c_int,
    pub multiplexing: ::std::os::raw::c_int,
    #[doc = " The following boolean flags are off by default"]
    pub disable_hardware_pulsing: bool,
    pub show_refresh_rate: bool,
    pub inverse_colors: bool,
    pub led_rgb_sequence: *const ::std::os::raw::c_char,
    pub pixel_mapper_config: *const ::std::os::raw::c_char,
    pub panel_type: *const ::std::os::raw::c_char,
    pub limit_refresh_rate_hz: ::std::os::raw::c_int,
    pub disable_busy_waiting: bool,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of RGBLedMatrixOptions"][::std::mem::size_of::<RGBLedMatrixOptions>() - 88usize];
    ["Alignment of RGBLedMatrixOptions"][::std::mem::align_of::<RGBLedMatrixOptions>() - 8usize];
    ["Offset of field: RGBLedMatrixOptions::hardware_mapping"]
        [::std::mem::offset_of!(RGBLedMatrixOptions, hardware_mapping) - 0usize];
    ["Offset of field: RGBLedMatrixOptions::rows"]
        [::std::mem::offset_of!(RGBLedMatrixOptions, rows) - 8usize];
    ["Offset of field: RGBLedMatrixOptions::cols"]
        [::std::mem::offset_of!(RGBLedMatrixOptions, cols) - 12usize];
    ["Offset of field: RGBLedMatrixOptions::chain_length"]
        [::std::mem::offset_of!(RGBLedMatrixOptions, chain_length) - 16usize];
    ["Offset of field: RGBLedMatrixOptions::parallel"]
        [::std::mem::offset_of!(RGBLedMatrixOptions, parallel) - 20usize];
    ["Offset of field: RGBLedMatrixOptions::pwm_bits"]
        [::std::mem::offset_of!(RGBLedMatrixOptions, pwm_bits) - 24usize];
    ["Offset of field: RGBLedMatrixOptions::pwm_lsb_nanoseconds"]
        [::std::mem::offset_of!(RGBLedMatrixOptions, pwm_lsb_nanoseconds) - 28usize];
    ["Offset of field: RGBLedMatrixOptions::pwm_dither_bits"]
        [::std::mem::offset_of!(RGBLedMatrixOptions, pwm_dither_bits) - 32usize];
    ["Offset of field: RGBLedMatrixOptions::brightness"]
        [::std::mem::offset_of!(RGBLedMatrixOptions, brightness) - 36usize];
    ["Offset of field: RGBLedMatrixOptions::scan_mode"]
        [::std::mem::offset_of!(RGBLedMatrixOptions, scan_mode) - 40usize];
    ["Offset of field: RGBLedMatrixOptions::row_address_type"]
        [::std::mem::offset_of!(RGBLedMatrixOptions, row_address_type) - 44usize];
    ["Offset of field: RGBLedMatrixOptions::multiplexing"]
        [::std::mem::offset_of!(RGBLedMatrixOptions, multiplexing) - 48usize];
    ["Offset of field: RGBLedMatrixOptions::disable_hardware_pulsing"]
        [::std::mem::offset_of!(RGBLedMatrixOptions, disable_hardware_pulsing) - 52usize];
    ["Offset of field: RGBLedMatrixOptions::show_refresh_rate"]
        [::std::mem::offset_of!(RGBLedMatrixOptions, show_refresh_rate) - 53usize];
    ["Offset of field: RGBLedMatrixOptions::inverse_colors"]
        [::std::mem::offset_of!(RGBLedMatrixOptions, inverse_colors) - 54usize];
    ["Offset of field: RGBLedMatrixOptions::led_rgb_sequence"]
        [::std::mem::offset_of!(RGBLedMatrixOptions, led_rgb_sequence) - 56usize];
    ["Offset of field: RGBLedMatrixOptions::pixel_mapper_config"]
        [::std::mem::offset_of!(RGBLedMatrixOptions, pixel_mapper_config) - 64usize];
    ["Offset of field: RGBLedMatrixOptions::panel_type"]
        [::std::mem::offset_of!(RGBLedMatrixOptions, panel_type) - 72usize];
    ["Offset of field: RGBLedMatrixOptions::limit_refresh_rate_hz"]
        [::std::mem::offset_of!(RGBLedMatrixOptions, limit_refresh_rate_hz) - 80usize];
    ["Offset of field: RGBLedMatrixOptions::disable_busy_waiting"]
        [::std::mem::offset_of!(RGBLedMatrixOptions, disable_busy_waiting) - 84usize];
};
#[doc = " Runtime options to simplify doing common things for many programs such as\n dropping privileges and becoming a daemon."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RGBLedRuntimeOptions {
    pub gpio_slowdown: ::std::os::raw::c_int,
    pub daemon: ::std::os::raw::c_int,
    pub drop_privileges: ::std::os::raw::c_int,
    pub do_gpio_init: bool,
    pub drop_priv_user: *const ::std::os::raw::c_char,
    pub drop_priv_group: *const ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of RGBLedRuntimeOptions"][::std::mem::size_of::<RGBLedRuntimeOptions>() - 32usize];
    ["Alignment of RGBLedRuntimeOptions"][::std::mem::align_of::<RGBLedRuntimeOptions>() - 8usize];
    ["Offset of field: RGBLedRuntimeOptions::gpio_slowdown"]
        [::std::mem::offset_of!(RGBLedRuntimeOptions, gpio_slowdown) - 0usize];
    ["Offset of field: RGBLedRuntimeOptions::daemon"]
        [::std::mem::offset_of!(RGBLedRuntimeOptions, daemon) - 4usize];
    ["Offset of field: RGBLedRuntimeOptions::drop_privileges"]
        [::std::mem::offset_of!(RGBLedRuntimeOptions, drop_privileges) - 8usize];
    ["Offset of field: RGBLedRuntimeOptions::do_gpio_init"]
        [::std::mem::offset_of!(RGBLedRuntimeOptions, do_gpio_init) - 12usize];
    ["Offset of field: RGBLedRuntimeOptions::drop_priv_user"]
        [::std::mem::offset_of!(RGBLedRuntimeOptions, drop_priv_user) - 16usize];
    ["Offset of field: RGBLedRuntimeOptions::drop_priv_group"]
        [::std::mem::offset_of!(RGBLedRuntimeOptions, drop_priv_group) - 24usize];
};
#[doc = " 24-bit RGB color."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Color"][::std::mem::size_of::<Color>() - 3usize];
    ["Alignment of Color"][::std::mem::align_of::<Color>() - 1usize];
    ["Offset of field: Color::r"][::std::mem::offset_of!(Color, r) - 0usize];
    ["Offset of field: Color::g"][::std::mem::offset_of!(Color, g) - 1usize];
    ["Offset of field: Color::b"][::std::mem::offset_of!(Color, b) - 2usize];
};
extern "C" {
    #[doc = " Universal way to create and initialize a matrix.\n The \"options\" struct (if not NULL) contains all default configuration values\n chosen by the programmer to create the matrix.\n\n If \"argc\" and \"argv\" are provided, this function also reads command line\n flags provided, that then can override any of the defaults given.\n The arguments that have been used from the command line are removed from\n the argv list (and argc is adjusted) - that way these don't mess with your\n own command line handling.\n\n The actual options used are filled back into the \"options\" struct if not\n NULL.\n\n Usage:\n ----------------\n int main(int argc, char **argv) {\n   struct RGBLedMatrixOptions options;\n   memset(&options, 0, sizeof(options));\n   options.rows = 32;            // You can set defaults if you want.\n   options.chain_length = 1;\n   struct RGBLedMatrix *matrix = led_matrix_create_from_options(&options,\n                                                                &argc, &argv);\n   if (matrix == NULL) {\n      led_matrix_print_flags(stderr);\n      return 1;\n   }\n   // do additional commandline handling; then use matrix...\n }\n ----------------"]
    pub fn led_matrix_create_from_options(
        options: *mut RGBLedMatrixOptions,
        argc: *mut ::std::os::raw::c_int,
        argv: *mut *mut *mut ::std::os::raw::c_char,
    ) -> *mut RGBLedMatrix;
}
extern "C" {
    pub fn led_matrix_create_from_options_const_argv(
        options: *mut RGBLedMatrixOptions,
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
    ) -> *mut RGBLedMatrix;
}
extern "C" {
    #[doc = " The way to completely initialize your matrix without using command line\n flags to initialize some things.\n\n The actual options used are filled back into the \"options\" and \"rt_options\"\n struct if not NULL. If they are null, the default value is used.\n\n Usage:\n ----------------\n int main(int argc, char **argv) {\n   struct RGBLedMatrixOptions options;\n   struct RGBLedRuntimeOptions rt_options;\n   memset(&options, 0, sizeof(options));\n   memset(&rt_options, 0, sizeof(rt_options));\n   options.rows = 32;            // You can set defaults if you want.\n   options.chain_length = 1;\n   rt_options.gpio_slowdown = 4;\n   struct RGBLedMatrix *matrix = led_matrix_create_from_options_and_rt_options(&options, &rt_options);\n   if (matrix == NULL) {\n      return 1;\n   }\n   // do additional commandline handling; then use matrix...\n }\n ----------------"]
    pub fn led_matrix_create_from_options_and_rt_options(
        opts: *mut RGBLedMatrixOptions,
        rt_opts: *mut RGBLedRuntimeOptions,
    ) -> *mut RGBLedMatrix;
}
// extern "C" {
    // #[doc = " Print available LED matrix options."]
    // pub fn led_matrix_print_flags(out: *mut FILE);
// }
extern "C" {
    #[doc = " Simple form of led_matrix_create_from_options() with just the few\n main options. Returns NULL if that was not possible.\n The \"rows\" are the number of rows supported by the display, so 32, 16 or 8.\n\n Number of \"chained_display\"s tells many of these are daisy-chained together\n (output of one connected to input of next).\n\n The \"parallel_display\" number determines if there is one or two displays\n connected in parallel to the GPIO port - this only works with newer\n Raspberry Pi that have 40 interface pins.\n\n This creates a realtime thread and requires root access to access the GPIO\n pins.\n So if you run this in a daemon, this should be called after becoming a\n daemon (as fork/exec stops threads) and before dropping privileges."]
    pub fn led_matrix_create(
        rows: ::std::os::raw::c_int,
        chained: ::std::os::raw::c_int,
        parallel: ::std::os::raw::c_int,
    ) -> *mut RGBLedMatrix;
}
extern "C" {
    #[doc = " Stop matrix and free memory.\n Always call before the end of the program to properly reset the hardware"]
    pub fn led_matrix_delete(matrix: *mut RGBLedMatrix);
}
extern "C" {
    #[doc = " Get active canvas from LED matrix for you to draw on.\n Ownership of returned pointer stays with the matrix, don't free()."]
    pub fn led_matrix_get_canvas(matrix: *mut RGBLedMatrix) -> *mut LedCanvas;
}
extern "C" {
    #[doc = " Return size of canvas."]
    pub fn led_canvas_get_size(
        canvas: *const LedCanvas,
        width: *mut ::std::os::raw::c_int,
        height: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    #[doc = " Set pixel at (x, y) with color (r,g,b)."]
    pub fn led_canvas_set_pixel(
        canvas: *mut LedCanvas,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        r: u8,
        g: u8,
        b: u8,
    );
}
extern "C" {
    #[doc = " Copies pixels to rectangle at (x, y) with size (width, height)."]
    pub fn led_canvas_set_pixels(
        canvas: *mut LedCanvas,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        colors: *mut Color,
    );
}
extern "C" {
    #[doc = " Clear screen (black)."]
    pub fn led_canvas_clear(canvas: *mut LedCanvas);
}
extern "C" {
    #[doc = " Fill matrix with given color."]
    pub fn led_canvas_fill(canvas: *mut LedCanvas, r: u8, g: u8, b: u8);
}
extern "C" {
    #[doc = " Create a new canvas to be used with led_matrix_swap_on_vsync()\n Ownership of returned pointer stays with the matrix, don't free()."]
    pub fn led_matrix_create_offscreen_canvas(matrix: *mut RGBLedMatrix) -> *mut LedCanvas;
}
extern "C" {
    #[doc = " Swap the given canvas (created with create_offscreen_canvas) with the\n currently active canvas on vsync (blocks until vsync is reached).\n Returns the previously active canvas. So with that, you can create double\n buffering:\n\n   struct LedCanvas *offscreen = led_matrix_create_offscreen_canvas(...);\n   led_canvas_set_pixel(offscreen, ...);   // not shown until swap-on-vsync\n   offscreen = led_matrix_swap_on_vsync(matrix, offscreen);\n   // The returned buffer, assigned to offscreen, is now the inactive buffer\n   // fill, then swap again."]
    pub fn led_matrix_swap_on_vsync(
        matrix: *mut RGBLedMatrix,
        canvas: *mut LedCanvas,
    ) -> *mut LedCanvas;
}
extern "C" {
    pub fn led_matrix_get_brightness(matrix: *mut RGBLedMatrix) -> u8;
}
extern "C" {
    pub fn led_matrix_set_brightness(matrix: *mut RGBLedMatrix, brightness: u8);
}
extern "C" {
    pub fn set_image(
        c: *mut LedCanvas,
        canvas_offset_x: ::std::os::raw::c_int,
        canvas_offset_y: ::std::os::raw::c_int,
        image_buffer: *const u8,
        buffer_size_bytes: usize,
        image_width: ::std::os::raw::c_int,
        image_height: ::std::os::raw::c_int,
        is_bgr: ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn load_font(bdf_font_file: *const ::std::os::raw::c_char) -> *mut LedFont;
}
extern "C" {
    pub fn baseline_font(font: *mut LedFont) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn height_font(font: *mut LedFont) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn create_outline_font(font: *mut LedFont) -> *mut LedFont;
}
extern "C" {
    pub fn delete_font(font: *mut LedFont);
}
extern "C" {
    pub fn draw_text(
        c: *mut LedCanvas,
        font: *mut LedFont,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        r: u8,
        g: u8,
        b: u8,
        utf8_text: *const ::std::os::raw::c_char,
        kerning_offset: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vertical_draw_text(
        c: *mut LedCanvas,
        font: *mut LedFont,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        r: u8,
        g: u8,
        b: u8,
        utf8_text: *const ::std::os::raw::c_char,
        kerning_offset: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn draw_circle(
        c: *mut LedCanvas,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        radius: ::std::os::raw::c_int,
        r: u8,
        g: u8,
        b: u8,
    );
}
extern "C" {
    pub fn draw_line(
        c: *mut LedCanvas,
        x0: ::std::os::raw::c_int,
        y0: ::std::os::raw::c_int,
        x1: ::std::os::raw::c_int,
        y1: ::std::os::raw::c_int,
        r: u8,
        g: u8,
        b: u8,
    );
}



// pub type __builtin_va_list = [__va_list_tag; 1usize];
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct __va_list_tag {
//     pub gp_offset: ::std::os::raw::c_uint,
//     pub fp_offset: ::std::os::raw::c_uint,
//     pub overflow_arg_area: *mut ::std::os::raw::c_void,
//     pub reg_save_area: *mut ::std::os::raw::c_void,
// }
// #[allow(clippy::unnecessary_operation, clippy::identity_op)]
// const _: () = {
//     ["Size of __va_list_tag"][::std::mem::size_of::<__va_list_tag>() - 24usize];
//     ["Alignment of __va_list_tag"][::std::mem::align_of::<__va_list_tag>() - 8usize];
//     ["Offset of field: __va_list_tag::gp_offset"]
//         [::std::mem::offset_of!(__va_list_tag, gp_offset) - 0usize];
//     ["Offset of field: __va_list_tag::fp_offset"]
//         [::std::mem::offset_of!(__va_list_tag, fp_offset) - 4usize];
//     ["Offset of field: __va_list_tag::overflow_arg_area"]
//         [::std::mem::offset_of!(__va_list_tag, overflow_arg_area) - 8usize];
//     ["Offset of field: __va_list_tag::reg_save_area"]
//         [::std::mem::offset_of!(__va_list_tag, reg_save_area) - 16usize];
// };

// pub const _STDINT_H: u32 = 1;
// pub const _FEATURES_H: u32 = 1;
// pub const _DEFAULT_SOURCE: u32 = 1;
// pub const __GLIBC_USE_ISOC2X: u32 = 0;
// pub const __USE_ISOC11: u32 = 1;
// pub const __USE_ISOC99: u32 = 1;
// pub const __USE_ISOC95: u32 = 1;
// pub const __USE_POSIX_IMPLICITLY: u32 = 1;
// pub const _POSIX_SOURCE: u32 = 1;
// pub const _POSIX_C_SOURCE: u32 = 200809;
// pub const __USE_POSIX: u32 = 1;
// pub const __USE_POSIX2: u32 = 1;
// pub const __USE_POSIX199309: u32 = 1;
// pub const __USE_POSIX199506: u32 = 1;
// pub const __USE_XOPEN2K: u32 = 1;
// pub const __USE_XOPEN2K8: u32 = 1;
// pub const _ATFILE_SOURCE: u32 = 1;
// pub const __WORDSIZE: u32 = 64;
// pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
// pub const __SYSCALL_WORDSIZE: u32 = 64;
// pub const __TIMESIZE: u32 = 64;
// pub const __USE_MISC: u32 = 1;
// pub const __USE_ATFILE: u32 = 1;
// pub const __USE_FORTIFY_LEVEL: u32 = 0;
// pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
// pub const __GLIBC_USE_DEPRECATED_SCANF: u32 = 0;
// pub const _STDC_PREDEF_H: u32 = 1;
// pub const __STDC_IEC_559__: u32 = 1;
// pub const __STDC_IEC_60559_BFP__: u32 = 201404;
// pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
// pub const __STDC_IEC_60559_COMPLEX__: u32 = 201404;
// pub const __STDC_ISO_10646__: u32 = 201706;
// pub const __GNU_LIBRARY__: u32 = 6;
// pub const __GLIBC__: u32 = 2;
// pub const __GLIBC_MINOR__: u32 = 35;
// pub const _SYS_CDEFS_H: u32 = 1;
// pub const __glibc_c99_flexarr_available: u32 = 1;
// pub const __LDOUBLE_REDIRECTS_TO_FLOAT128_ABI: u32 = 0;
// pub const __HAVE_GENERIC_SELECTION: u32 = 1;
// pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
// pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
// pub const __GLIBC_USE_IEC_60559_BFP_EXT_C2X: u32 = 0;
// pub const __GLIBC_USE_IEC_60559_EXT: u32 = 0;
// pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
// pub const __GLIBC_USE_IEC_60559_FUNCS_EXT_C2X: u32 = 0;
// pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
// pub const _BITS_TYPES_H: u32 = 1;
// pub const _BITS_TYPESIZES_H: u32 = 1;
// pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
// pub const __INO_T_MATCHES_INO64_T: u32 = 1;
// pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
// pub const __STATFS_MATCHES_STATFS64: u32 = 1;
// pub const __KERNEL_OLD_TIMEVAL_MATCHES_TIMEVAL64: u32 = 1;
// pub const __FD_SETSIZE: u32 = 1024;
// pub const _BITS_TIME64_H: u32 = 1;
// pub const _BITS_WCHAR_H: u32 = 1;
// pub const _BITS_STDINT_INTN_H: u32 = 1;
// pub const _BITS_STDINT_UINTN_H: u32 = 1;
// pub const INT8_MIN: i32 = -128;
// pub const INT16_MIN: i32 = -32768;
// pub const INT32_MIN: i32 = -2147483648;
// pub const INT8_MAX: u32 = 127;
// pub const INT16_MAX: u32 = 32767;
// pub const INT32_MAX: u32 = 2147483647;
// pub const UINT8_MAX: u32 = 255;
// pub const UINT16_MAX: u32 = 65535;
// pub const UINT32_MAX: u32 = 4294967295;
// pub const INT_LEAST8_MIN: i32 = -128;
// pub const INT_LEAST16_MIN: i32 = -32768;
// pub const INT_LEAST32_MIN: i32 = -2147483648;
// pub const INT_LEAST8_MAX: u32 = 127;
// pub const INT_LEAST16_MAX: u32 = 32767;
// pub const INT_LEAST32_MAX: u32 = 2147483647;
// pub const UINT_LEAST8_MAX: u32 = 255;
// pub const UINT_LEAST16_MAX: u32 = 65535;
// pub const UINT_LEAST32_MAX: u32 = 4294967295;
// pub const INT_FAST8_MIN: i32 = -128;
// pub const INT_FAST16_MIN: i64 = -9223372036854775808;
// pub const INT_FAST32_MIN: i64 = -9223372036854775808;
// pub const INT_FAST8_MAX: u32 = 127;
// pub const INT_FAST16_MAX: u64 = 9223372036854775807;
// pub const INT_FAST32_MAX: u64 = 9223372036854775807;
// pub const UINT_FAST8_MAX: u32 = 255;
// pub const UINT_FAST16_MAX: i32 = -1;
// pub const UINT_FAST32_MAX: i32 = -1;
// pub const INTPTR_MIN: i64 = -9223372036854775808;
// pub const INTPTR_MAX: u64 = 9223372036854775807;
// pub const UINTPTR_MAX: i32 = -1;
// pub const PTRDIFF_MIN: i64 = -9223372036854775808;
// pub const PTRDIFF_MAX: u64 = 9223372036854775807;
// pub const SIG_ATOMIC_MIN: i32 = -2147483648;
// pub const SIG_ATOMIC_MAX: u32 = 2147483647;
// pub const SIZE_MAX: i32 = -1;
// pub const WINT_MIN: u32 = 0;
// pub const WINT_MAX: u32 = 4294967295;
// pub const _STDIO_H: u32 = 1;
// pub const __GNUC_VA_LIST: u32 = 1;
// pub const _____fpos_t_defined: u32 = 1;
// pub const ____mbstate_t_defined: u32 = 1;
// pub const _____fpos64_t_defined: u32 = 1;
// pub const ____FILE_defined: u32 = 1;
// pub const __FILE_defined: u32 = 1;
// pub const __struct_FILE_defined: u32 = 1;
// pub const _IO_EOF_SEEN: u32 = 16;
// pub const _IO_ERR_SEEN: u32 = 32;
// pub const _IO_USER_LOCK: u32 = 32768;
// pub const _IOFBF: u32 = 0;
// pub const _IOLBF: u32 = 1;
// pub const _IONBF: u32 = 2;
// pub const BUFSIZ: u32 = 8192;
// pub const EOF: i32 = -1;
// pub const SEEK_SET: u32 = 0;
// pub const SEEK_CUR: u32 = 1;
// pub const SEEK_END: u32 = 2;
// pub const P_tmpdir: &[u8; 5] = b"/tmp\0";
// pub const _BITS_STDIO_LIM_H: u32 = 1;
// pub const L_tmpnam: u32 = 20;
// pub const TMP_MAX: u32 = 238328;
// pub const FILENAME_MAX: u32 = 4096;
// pub const L_ctermid: u32 = 9;
// pub const FOPEN_MAX: u32 = 16;
// pub const __HAVE_FLOAT128: u32 = 0;
// pub const __HAVE_DISTINCT_FLOAT128: u32 = 0;
// pub const __HAVE_FLOAT64X: u32 = 1;
// pub const __HAVE_FLOAT64X_LONG_DOUBLE: u32 = 1;
// pub const __HAVE_FLOAT16: u32 = 0;
// pub const __HAVE_FLOAT32: u32 = 1;
// pub const __HAVE_FLOAT64: u32 = 1;
// pub const __HAVE_FLOAT32X: u32 = 1;
// pub const __HAVE_FLOAT128X: u32 = 0;
// pub const __HAVE_DISTINCT_FLOAT16: u32 = 0;
// pub const __HAVE_DISTINCT_FLOAT32: u32 = 0;
// pub const __HAVE_DISTINCT_FLOAT64: u32 = 0;
// pub const __HAVE_DISTINCT_FLOAT32X: u32 = 0;
// pub const __HAVE_DISTINCT_FLOAT64X: u32 = 0;
// pub const __HAVE_DISTINCT_FLOAT128X: u32 = 0;
// pub const __HAVE_FLOATN_NOT_TYPEDEF: u32 = 0;
// pub const true_: u32 = 1;
// pub const false_: u32 = 0;
// pub const __bool_true_false_are_defined: u32 = 1;
// pub type __u_char = ::std::os::raw::c_uchar;
// pub type __u_short = ::std::os::raw::c_ushort;
// pub type __u_int = ::std::os::raw::c_uint;
// pub type __u_long = ::std::os::raw::c_ulong;
// pub type __int8_t = ::std::os::raw::c_schar;
// pub type __uint8_t = ::std::os::raw::c_uchar;
// pub type __int16_t = ::std::os::raw::c_short;
// pub type __uint16_t = ::std::os::raw::c_ushort;
// pub type __int32_t = ::std::os::raw::c_int;
// pub type __uint32_t = ::std::os::raw::c_uint;
// pub type __int64_t = ::std::os::raw::c_long;
// pub type __uint64_t = ::std::os::raw::c_ulong;
// pub type __int_least8_t = __int8_t;
// pub type __uint_least8_t = __uint8_t;
// pub type __int_least16_t = __int16_t;
// pub type __uint_least16_t = __uint16_t;
// pub type __int_least32_t = __int32_t;
// pub type __uint_least32_t = __uint32_t;
// pub type __int_least64_t = __int64_t;
// pub type __uint_least64_t = __uint64_t;
// pub type __quad_t = ::std::os::raw::c_long;
// pub type __u_quad_t = ::std::os::raw::c_ulong;
// pub type __intmax_t = ::std::os::raw::c_long;
// pub type __uintmax_t = ::std::os::raw::c_ulong;
// pub type __dev_t = ::std::os::raw::c_ulong;
// pub type __uid_t = ::std::os::raw::c_uint;
// pub type __gid_t = ::std::os::raw::c_uint;
// pub type __ino_t = ::std::os::raw::c_ulong;
// pub type __ino64_t = ::std::os::raw::c_ulong;
// pub type __mode_t = ::std::os::raw::c_uint;
// pub type __nlink_t = ::std::os::raw::c_ulong;
// pub type __off_t = ::std::os::raw::c_long;
// pub type __off64_t = ::std::os::raw::c_long;
// pub type __pid_t = ::std::os::raw::c_int;
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct __fsid_t {
//     pub __val: [::std::os::raw::c_int; 2usize],
// }
// #[allow(clippy::unnecessary_operation, clippy::identity_op)]
// const _: () = {
//     ["Size of __fsid_t"][::std::mem::size_of::<__fsid_t>() - 8usize];
//     ["Alignment of __fsid_t"][::std::mem::align_of::<__fsid_t>() - 4usize];
//     ["Offset of field: __fsid_t::__val"][::std::mem::offset_of!(__fsid_t, __val) - 0usize];
// };
// pub type __clock_t = ::std::os::raw::c_long;
// pub type __rlim_t = ::std::os::raw::c_ulong;
// pub type __rlim64_t = ::std::os::raw::c_ulong;
// pub type __id_t = ::std::os::raw::c_uint;
// pub type __time_t = ::std::os::raw::c_long;
// pub type __useconds_t = ::std::os::raw::c_uint;
// pub type __suseconds_t = ::std::os::raw::c_long;
// pub type __suseconds64_t = ::std::os::raw::c_long;
// pub type __daddr_t = ::std::os::raw::c_int;
// pub type __key_t = ::std::os::raw::c_int;
// pub type __clockid_t = ::std::os::raw::c_int;
// pub type __timer_t = *mut ::std::os::raw::c_void;
// pub type __blksize_t = ::std::os::raw::c_long;
// pub type __blkcnt_t = ::std::os::raw::c_long;
// pub type __blkcnt64_t = ::std::os::raw::c_long;
// pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
// pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
// pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
// pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
// pub type __fsword_t = ::std::os::raw::c_long;
// pub type __ssize_t = ::std::os::raw::c_long;
// pub type __syscall_slong_t = ::std::os::raw::c_long;
// pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
// pub type __loff_t = __off64_t;
// pub type __caddr_t = *mut ::std::os::raw::c_char;
// pub type __intptr_t = ::std::os::raw::c_long;
// pub type __socklen_t = ::std::os::raw::c_uint;
// pub type __sig_atomic_t = ::std::os::raw::c_int;
// pub type int_least8_t = __int_least8_t;
// pub type int_least16_t = __int_least16_t;
// pub type int_least32_t = __int_least32_t;
// pub type int_least64_t = __int_least64_t;
// pub type uint_least8_t = __uint_least8_t;
// pub type uint_least16_t = __uint_least16_t;
// pub type uint_least32_t = __uint_least32_t;
// pub type uint_least64_t = __uint_least64_t;
// pub type int_fast8_t = ::std::os::raw::c_schar;
// pub type int_fast16_t = ::std::os::raw::c_long;
// pub type int_fast32_t = ::std::os::raw::c_long;
// pub type int_fast64_t = ::std::os::raw::c_long;
// pub type uint_fast8_t = ::std::os::raw::c_uchar;
// pub type uint_fast16_t = ::std::os::raw::c_ulong;
// pub type uint_fast32_t = ::std::os::raw::c_ulong;
// pub type uint_fast64_t = ::std::os::raw::c_ulong;
// pub type intmax_t = __intmax_t;
// pub type uintmax_t = __uintmax_t;
// pub type va_list = __builtin_va_list;
// pub type __gnuc_va_list = __builtin_va_list;


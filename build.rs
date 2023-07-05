fn main() {
    println!("cargo:rustc-link-search=native=C:/SDL2SDK/SDL2-2.28.0/lib/x64");
    println!("cargo:rustc-link-search=native=./libs/libcimgui-sys/deps/cimgui/build/Debug");
}
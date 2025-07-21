fn main() {
    // This tells Cargo to recompile if our templates change
    println!("cargo:rerun-if-changed=templates");
}

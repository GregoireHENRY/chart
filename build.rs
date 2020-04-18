use std::env::var;

fn main()
{
    let relative = var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search={}/usr/dll", relative);
}

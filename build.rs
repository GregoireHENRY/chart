fn main()
{
    //println!("cargo::rustc-link-search=native=/usr/lib/x86_64-linux-gnu/");
    //println!("cargo::rustc-link-lib=static=SDL2");
    println!("cargo:rustc-link-search=crate=usr/dll");
}

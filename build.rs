fn main()
{
    let root = env!("CARGO_MANIFEST_DIR");
    println!("cargo:rustc-env=LD_LIBRARY_PATH={}/usr/dll", root);
    println!("cargo:rustc-link-search={}/usr/dll", root);
    println!("cargo:rustc-link-lib=SDL2");
}

/*
SDL2
SDL2_ttf
freetype-6
zlib1
*/

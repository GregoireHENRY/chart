fn main()
{
    #[cfg(feature="static")]
    {
        //println!("cargo::rustc-link-search=native=/usr/lib/x86_64-linux-gnu/");
        println!("cargo::rustc-link-lib=static=SDL2");
    }
}

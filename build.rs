extern crate gcc;

fn main() {
    gcc::Config::new()
        // Compilation options
        .flag("-fPIC")
        .flag("-fno-rtti")
        .flag("-fno-exceptions")
        .flag("-O3")
        // #defines
        .define("NDEBUG", None)
        .define("BLARGG_NONPORTABLE", None)
        // The codes
        .file("src/snes_spc/dsp.cpp")
        .file("src/snes_spc/SNES_SPC.cpp")
        .file("src/snes_spc/SNES_SPC_misc.cpp")
        .file("src/snes_spc/SNES_SPC_state.cpp")
        .file("src/snes_spc/spc.cpp")
        .file("src/snes_spc/SPC_DSP.cpp")
        .file("src/snes_spc/SPC_Filter.cpp")
        // Output
        .compile("libsnes_spc.a");
}

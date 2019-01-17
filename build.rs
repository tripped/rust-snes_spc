extern crate cc;

fn main() {
    cc::Build::new()
        // Compilation options
        .flag("-fPIC")
        .flag("-fno-rtti")
        .flag("-fno-exceptions")
        .flag("-O3")
        // The C code is a dumpster fire, so disable a bunch of warnings
        .flag("-Wno-implicit-fallthrough")
        .flag("-Wno-shift-negative-value")
        .flag("-Wno-array-bounds") // oh no
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

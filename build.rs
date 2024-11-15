use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut build = cc::Build::new();
    // Default optimization level
    build.opt_level(3);

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_else(|_| "unknown".to_string());
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_else(|_| "unknown".to_string());
    println!("Target OS: {}", target_os);
    println!("Target Arch: {}", target_arch);
    // Base configuration
    build
        .flag("-fwrapv")
        .flag("-fomit-frame-pointer")
        .flag("-march=native")
        .flag("-Wno-sign-compare");

    // Architecture-specific settings
    let mut clang_arg_arch = "";
    match target_arch.as_str() {
        "x86_64" => {
            build.define("_AMD64_", None);
            clang_arg_arch = "-D_AMD64_";

            // Handle ASM, AVX, and AVX2 features
            let use_generic = env::var("GENERIC").map(|v| v == "TRUE").unwrap_or(false);
            let use_asm = env::var("ASM").map(|v| v == "TRUE").unwrap_or(!use_generic);
            let use_avx = env::var("AVX").map(|v| v == "TRUE").unwrap_or(!use_generic);
            let use_avx2 = env::var("AVX2")
                .map(|v| v == "TRUE")
                .unwrap_or(!use_generic);

            if use_asm {
                build.define("_ASM_", None);
            }
            if use_avx {
                build.define("_AVX_", None).flag("-mavx");
            }
            if use_avx2 {
                build.define("_AVX2_", None).flag("-mavx2");
            }
            if use_generic {
                build.define("_GENERIC_", None);
            }

            // Add assembly files based on features
            if use_asm {
                if use_avx2 {
                    // Generate consts.s from consts.c
                    // let out_dir = env::var("OUT_DIR").unwrap();
                    // let consts_s = Path::new(&out_dir).join("consts.s");

                    cc::Build::new()
                        .file("AMD64/consts.c")
                        .flag("-S")
                        .compile("consts");

                    // Add AVX2 assembly
                    build.file("AMD64/fp2_1271_AVX2.S");
                } else {
                    build.file("AMD64/fp2_1271.S");
                }
            }
        }
        "aarch64" => {
            clang_arg_arch = "-D_ARM64_";
            build.define("_ARM64_", None);
            if env::var("GENERIC").map(|v| v == "TRUE").unwrap_or(false) {
                build.define("_GENERIC_", None);
            }
        }
        "arm" => {
            build.define("_ARM_", None);
            clang_arg_arch = "-D_ARM_";
        }
        "x86" => {
            build.define("_X86_", None);
            clang_arg_arch = "-D_X86_";
        }
        "wasm32" => {
            build.define("_WASM_", None);
        }
        _ => {
            build.define("_GENERIC_", None);
        }
    };

    let mut clang_args = Vec::with_capacity(3);
    clang_args.push("-Ifourq");
    // Link with lib
    println!("cargo:rustc-link-lib=fourq");
    // Common defines
    if target_os == "linux" {
        build.define("__LINUX__", None);
        clang_args.push("-D__LINUX__");
        clang_args.push(clang_arg_arch);
    }
    if target_os == "macos" {
        build.define("__APPLE__", None).define("__MACH__", None);
        build.flag("-Wno-bitwise-instead-of-logical");
    }

    // Optional features
    if env::var("USE_ENDO").map(|v| v != "FALSE").unwrap_or(true) {
        build.define("USE_ENDO", None);
    }

    if env::var("SERIAL_PUSH")
        .map(|v| v == "TRUE")
        .unwrap_or(false)
    {
        build.define("PUSH_SET", None);
    }

    // Add source files
    let sources = [
        "fourq/eccp2.c",
        "fourq/eccp2_no_endo.c",
        "fourq/eccp2_core.c",
        "fourq/crypto_util.c",
        "fourq/schnorrq.c",
        "fourq/hash_to_curve.c",
        "fourq/kex.c",
        "fourq/sha512.c",
        "fourq/random.c",
    ];

    for source in &sources {
        build.file(source);
    }

    // Handle shared library compilation if requested
    if env::var("SHARED_LIB").map(|v| v == "TRUE").unwrap_or(false) {
        build.pic(true);
    }

    // Compile everything
    build.compile("fourq");

    // Add rerun-if-changed directives for all source files
    for source in &sources {
        println!("cargo:rerun-if-changed={}", source);
    }
    println!("cargo:rerun-if-changed=ARM64/fp_arm64.h");
    println!("cargo:rerun-if-changed=AMD64/fp2_1271.S");
    println!("cargo:rerun-if-changed=AMD64/fp2_1271_AVX2.S");
    println!("cargo:rerun-if-changed=AMD64/consts.c");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_ARCH");
    println!("cargo:rerun-if-env-changed=GENERIC");
    println!("cargo:rerun-if-env-changed=ASM");
    println!("cargo:rerun-if-env-changed=AVX");
    println!("cargo:rerun-if-env-changed=AVX2");
    println!("cargo:rerun-if-env-changed=USE_ENDO");
    println!("cargo:rerun-if-env-changed=SERIAL_PUSH");
    println!("cargo:rerun-if-env-changed=EXTENDED_SET");
    println!("cargo:rerun-if-env-changed=SHARED_LIB");

    // Configure and generate bindings
    let out_path = Path::new(&out_dir).join("bindings.rs");
    let bindings = bindgen::Builder::default()
        .header("fourq/FourQ_internal.h")
        // Add the include path for the FourQ headers
        .clang_args(clang_args)
        .use_core()
        // Make bindings derive useful traits
        .derive_debug(true)
        .derive_default(true)
        .derive_eq(true)
        .derive_hash(true)
        // Allow all FourQ functions
        .allowlist_function("ecc.*")
        .allowlist_function("SchnorrQ.*")
        .allowlist_function("Compressed.*")
        .allowlist_function("PublicKey.*")
        .allowlist_function("KeyGeneration")
        .allowlist_function("SecretAgreement")
        .allowlist_function("HashToCurve")
        .allowlist_function("to_Montgomery")
        .allowlist_function("from_Montgomery")
        .allowlist_function("Montgomery_.*")
        .allowlist_function("add_mod_order")
        .allowlist_function("subtract_mod_order")
        .allowlist_function("modulo_order")
        // Add internal functions
        .allowlist_function("mod1271")
        .allowlist_function("encode")
        .allowlist_function("decode")
        .allowlist_function("fp.*") // Allow all fp_* functions
        .allowlist_function("mp_.*") // Allow all mp_* functions
        .allowlist_function("is_zero_ct")
        .allowlist_function("subtract")
        .allowlist_function("clear_words")
        // Allow FourQ types
        .allowlist_type("point_t")
        .allowlist_type("digit_t")
        .allowlist_type("f2elm_t")
        .allowlist_type("felm_t")
        .allowlist_type("point_extproj")
        .allowlist_type("point_extproj_precomp")
        .allowlist_type("point_precomp")
        .allowlist_function("R1_to_R2")
        .allowlist_function("R1_to_R3")
        .allowlist_type("ECCRYPTO_STATUS")
        // Generate constants
        .generate_comments(true)
        // .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Generate
        .generate()
        .expect("Unable to generate bindings");

    // Write bindings to bindings.rs in the output directory
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");

    // Tell cargo to rebuild if headers change
    println!("cargo:rerun-if-changed=fourq/FourQ_internal.h");
    println!("cargo:rerun-if-changed=fourq/FourQ_api.h");
    println!("cargo:rerun-if-changed=fourq/FourQ.h");
}
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("gxx_personality_v0_stub.c");
    fs::write(
        &dest_path,
        "
        #include \"unwind.h\"
        #include \"stdint.h\"
        
        _Unwind_Reason_Code __gxx_personality_v0(int version, _Unwind_Action actions, uint64_t exceptionClass, _Unwind_Exception * unwind_exception, struct _Unwind_Context * context) {
            return _URC_NO_REASON;
        }
        "
    ).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-arg={}", dest_path.display());
}
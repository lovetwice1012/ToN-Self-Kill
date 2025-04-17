fn main() {
    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .csharp_dll_name("ToN-Self-Kill")
        .generate_csharp_file("../dotnet/NativeMethods.g.cs")
        .unwrap();
}
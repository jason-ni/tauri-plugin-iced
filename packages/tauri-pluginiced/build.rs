fn main() {
    tauri_plugin::Builder::new(&[])
        .global_api_script_path("./api-iife.js")
        .android_path("android")
        .ios_path("ios");
}

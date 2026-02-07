// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ICD {
    library_path: String,
    api_version: String,
    is_portability_driver: bool,
}

#[derive(Serialize, Deserialize)]
struct VulkanICDNames {
    file_format_version: String,
    #[serde(rename = "ICD")]
    icd: ICD,
}

fn main() {
    std::env::set_var("WGPU_BACKEND", "vulkan");
    let executable_path = std::env::current_exe().unwrap();
    let executable_parent_path = executable_path.parent().unwrap().parent().unwrap();
    let framework_path = executable_parent_path.join("Frameworks");
    // Create a temporary directory to store the icd file.
    let tmp_dir = std::env::temp_dir().join("tauri-iced-vulkan");
    std::fs::create_dir_all(&tmp_dir).unwrap();
    let vk_id_filenames_path = tmp_dir.join("MoltenVK_icd.json");

    if !vk_id_filenames_path.exists() {
        // Create VulkanICDNames and write it to the MoltenVK_icd.json file.
        let library_path = framework_path.join("libMoltenVK.dylib");
        let icd = VulkanICDNames {
            file_format_version: String::from("1.0.0"),
            icd: ICD {
                library_path: library_path.to_str().unwrap().to_string(),
                api_version: String::from("1.4.0"),
                is_portability_driver: true,
            },
        };
        let file = std::fs::File::create(&vk_id_filenames_path).unwrap();
        serde_json::to_writer_pretty(file, &icd).unwrap();
    }
    std::env::set_var("VK_ICD_FILENAMES", vk_id_filenames_path.to_str().unwrap());
    app_lib::run();
}

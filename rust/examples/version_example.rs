// #exonware/xwsystem/rust/examples/version_example.rs
//! Example usage of the version module

use xwsystem_rust::version;

fn main() {
    println!("XWSystem Rust Version Example");
    println!("============================\n");
    
    // Get version string
    println!("Version: {}", version::get_version());
    
    // Get version components
    let (major, minor, patch, build) = version::get_version_info();
    println!("Version Components:");
    println!("  Major: {}", major);
    println!("  Minor: {}", minor);
    println!("  Patch: {}", patch);
    if let Some(b) = build {
        println!("  Build: {}", b);
    } else {
        println!("  Build: None (release)");
    }
    
    // Get version info struct
    let info = version::get_version_dict();
    println!("\nVersion Info Struct:");
    println!("  {:?}", info);
    
    // Check version type
    println!("\nVersion Type:");
    if version::is_dev_version() {
        println!("  Development version");
    } else {
        println!("  Release version");
    }
    
    if version::is_release_version() {
        println!("  (Release)");
    } else {
        println!("  (Development)");
    }
    
    // Serialize to JSON
    println!("\nVersion Info as JSON:");
    let json = serde_json::to_string_pretty(&info).unwrap();
    println!("{}", json);
}


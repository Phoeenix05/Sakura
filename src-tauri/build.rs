// TODO: add the remainen architectures
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
const ARCH_TAG: &'static str = "darwin_arm64";

fn main() {
    let client = reqwest::blocking::Client::new();

    let request = client
        .get("https://api.github.com/repos/pocketbase/pocketbase/releases")
        .header("user-agent", "aemil.dev/Sakura 0.1.0/build.rs")
        .header("accept", "application/vnd.github+json")
        .header("x-github-api-version", "2022-11-28");

    let response = request.send().unwrap();
    let data: serde_json::Value = response.json().unwrap();

    let release = data
        .as_array()
        .unwrap()
        .into_iter()
        .find(|release| release["tag_name"].eq("v0.22.10"))
        .unwrap();

    let asset = release["assets"]
        .as_array()
        .unwrap()
        .into_iter()
        .find(|asset| asset["name"].as_str().unwrap().contains(ARCH_TAG))
        .unwrap();

    // is a url to a zip file
    let asset_url = asset["browser_download_url"].as_str().unwrap();
    let asset_name = asset_url.split('/').last().unwrap();

    let data = reqwest::blocking::Client::new()
        .get(asset_url)
        .send()
        .unwrap()
        .bytes()
        .unwrap();

    let path = std::env::temp_dir().join("dev.aemil.sakura/assets");
    std::fs::create_dir_all(&path).unwrap();

    use std::io::Write as _;
    let path = path.join(asset_name);
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(&data).unwrap();

    tauri_build::build()
}

/// **params**
/// * `path`: eg. `/manga`, `/chapter`, etc
/// * `filters`: valid filters can be found [here](https://api.mangadex.org/docs/swagger.html#/Manga/get-search-manga).
///
/// **example**
/// ```ts
/// import { invoke } from '@tauri-apps/api'
///
/// await invoke<string>('construct_url', { ... })
/// ```
///
/// **NOTE:** command handler is defined in main.rs
#[tauri::command]
pub fn construct_url(
    mut path: String,
    filters: std::collections::HashMap<String, String>,
) -> String {
    if path.starts_with("/") {
        path.remove(0);
    }

    let query_params = filters
        .into_iter()
        .map(|(key, val)| format!("{key}={val}"))
        .collect::<Vec<_>>()
        .join("&");

    format!("https://api.mangadex.org/{path}?{query_params}")
}

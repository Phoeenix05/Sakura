// #[derive(serde::Serialize, serde::Deserialize)]
// struct Response {
//     titles: Vec<String>,
//     ids: Vec<String>,
// }

#[tauri::command]
pub async fn manga_data(uuid: String) -> String {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("https://api.mangadex.org/manga/{uuid}/feed"))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    response
}

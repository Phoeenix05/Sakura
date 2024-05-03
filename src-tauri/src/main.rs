// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod window;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();

async fn create_pool(config: std::sync::Arc<tauri::Config>) {
    use sqlx::migrate::MigrateDatabase as _;

    let data_path = tauri::api::path::app_data_dir(&config).unwrap();
    let db_url = format!(
        "sqlite:{}/data.sqlite",
        data_path.to_string_lossy().trim_matches('"')
    );

    if !sqlx::Sqlite::database_exists(&db_url).await.unwrap() {
        sqlx::Sqlite::create_database(&db_url).await.unwrap();
    }

    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_lazy(&db_url)
        .unwrap();

    MIGRATOR.run(&pool).await.unwrap();
}

fn main() {
    let setup = |app: &mut tauri::App| {
        let config = app.config();
        tauri::async_runtime::spawn(create_pool(config));

        #[cfg(target_os = "macos")]
        crate::window::macos::setup_mac_window(app);
        #[cfg(target_os = "windows")]
        crate::window::windows::setup_win_window(app);

        Ok(())
    };

    tauri::Builder::default()
        .setup(setup)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use std::path::PathBuf;

use surrealdb::{
    engine::local::{Db, SpeeDb},
    Surreal,
};
use tauri::Config;

pub struct SakuraState {
    config: Config,
    app_dir: PathBuf,
    data_dir: PathBuf,
    db: Surreal<Db>,
}

impl SakuraState {
    pub async fn init() -> Self {
        #[allow(non_upper_case_globals)]
        const contents: &str = include_str!("../tauri.conf.json");
        let config: Config = serde_json::from_str(contents).unwrap();

        // should be simething like "~/Library/Application Support/dev.aemil.sakura"
        // and on windows it should be somewhere in "%AppData%" folder.
        let app_dir = tauri::api::path::app_data_dir(&config).unwrap();

        let mut data_dir = app_dir.clone();
        data_dir.push("./data");

        let db = Surreal::new::<SpeeDb>(data_dir.to_str().unwrap())
            .await
            .unwrap();

        Self {
            config,
            app_dir,
            data_dir,
            db,
        }
    }
}

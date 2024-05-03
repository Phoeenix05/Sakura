macro_rules! pb_sidecar {
    ($args:expr) => {
        let pb = tauri::api::process::Command::new_sidecar("pocketbase")
            .expect("failed to create `pocketbase` binary command")
            .current_dir(tauri::api::path::app_data_dir(&tauri::Config::default()).unwrap());
        pb.args($args).spawn().expect("failed to spawn sidecar");
    };
}
pub(crate) use pb_sidecar;

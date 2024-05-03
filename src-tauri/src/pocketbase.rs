macro_rules! pb_sidecar {
    ($args:expr) => {
        let _pb = tauri::api::process::Command::new_sidecar("pocketbase")
            .expect("failed to create `pocketbase` binary command")
            .args($args)
            .spawn()
            .expect("failed to spawn sidecar");
    };
}
pub(crate) use pb_sidecar;

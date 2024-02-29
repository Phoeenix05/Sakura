cargo *rest:
  cd src-tauri && cargo add {{ rest }}

test-release:
  pnpm tauri dev --release --features tauri/devtools

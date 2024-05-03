#!/usr/bin/env sh

repo="pocketbase/pocketbase"
dir="src-tauri/pocketbase"
system=$(uname -om | awk '{gsub(" ", "_", $0); print tolower($0)}')
target_triple=$(rustc -vV | rg 'host:' | awk '{gsub("host: ", "", $0); print $0}')

echo "\n"
echo "\033[1m repo\033[0m: \t\t\t$repo"
echo "\033[1m install directory\033[0m: \t$dir"
echo "\033[1m system tag\033[0m: \t\t$system"
echo "\033[1m target triple\033[0m: \t$target_triple"
echo "\n"

echo "=> fetching release info from github $repo"
download_url=$(curl -L \
  -H "Accept: application/vnd.github+json" \
  -H "User-Agent: dev.aemil/Sakura" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  https://api.github.com/repos/$repo/releases \
  | jq '.[] | select(.tag_name == "v0.22.10") | .assets' \
  | jq -r --arg system "$system" '.[] | select(.name | contains($system)) | .browser_download_url'
)

echo "=> downloading $download_url"
curl -L -o pocketbase.zip "$download_url"

echo "=> creating asset directory for pocketbase"
mkdir -p $dir

echo "=> extracting pocketbase binary"
unzip -od $dir pocketbase.zip pocketbase

echo "=> renaming pocketbase binary to include target triple"
mv $dir/pocketbase $dir/pocketbase-$target_triple

echo "=> cleaning up"
rm pocketbase.zip

<p align="center">
    <img src="https://socialify.git.ci/phoeenix05/mangadex-reader/image?issues=1&language=1&name=1&owner=1&pattern=Circuit%20Board&stargazers=1&theme=Auto"/>
</p>

## 1. Introduction
This is the project that I'm currently working on in my freetime. The aim of this application / project is to provide a desktop application to read mangas. 

The UI for the application won't be looking that good at the start but I will try to improve it bit by bit overtime.

## 2. Prerequisites
> cargo, rust, pnpm

MacOS (this works on linux too if you use homebrew)
```zsh
brew install cargo pnpm
```

Right now for windows or manual installation without homebrew consult the installation guides for the dependencies.
```zsh
# On MacOS, Linux, or another Unix-like OS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
curl -fsSL https://get.pnpm.io/install.sh | sh -
```

NOTE: This application requires sqlite3 to be installed to work. SQLite should be already installed on MacOS and Windows by default.

<!-- For Crystal installation guides check their [installation page](https://crystal-lang.org/install/) or from latest source on [GitHub](https://github.com/crystal-lang/crystal/releases/latest). -->

**NOTE:** I am using a MacBook so most of the tooling and installation guides are for macos right now. So if you do want to contribute or change something you'll either need to figure out some things or you can ask me for help.

## 3. Technologies used and reasoning
![Static Badge](https://img.shields.io/badge/tauri-333333?style=for-the-badge&logo=tauri&link=https%3A%2F%2Ftauri.app)
![Static Badge](https://img.shields.io/badge/solidjs-333333?style=for-the-badge&logo=solid&logoColor=rgb(95%2C%20136%2C%20195)&link=https%3A%2F%2Fsolidjs.com)
![Static Badge](https://img.shields.io/badge/sqlite3-white?style=for-the-badge&logo=sqlite&color=333333)

- **Tauri** because it is much more lightweight than **electron**.
- **SolidJS** because it is faster than **React**, in my opinion atleast.
- **SQLite3** because it was basically the only self contained database I could find.

## 4. Installation / Development

- **Step 1**: Clone the repository
```zsh
git clone https://github.com/phoeenix05/mangadex-reader
# or
gh repo clone phoeenix05/mangadex-reader
```

- **Step 1.5**: Init submodules
```zsh
git submodule init
```

- **Step 2**: Running the project
```zsh 
pnpm dev
npm run dev
yarn run dev
```

- **Step 4**: Building the project
```zsh 
pnpm build
npm run build
yarn run build
# or
cargo tauri build
```

## 5. Contribution guidelines
None. If you want to contribute just fork the repository make your changes and create a pull request. If there are things I would like to change in that pull request then I'll merge it and change them.

## 6. Licensing
I am completely free and licensed under the [MIT license](https://github.com/Phoeenix05/mangadex-reader/blob/main/LICENSE). But if you like, you can feed me with a star on [GitHub](https://github.com/Phoeenix05/mangadex-reader).


<picture>
    <source media="(prefers-color-scheme: dark)" srcset="resources/banners/dark.svg" />
    <source media="(prefers-color-scheme: light)" srcset="resources/banners/light.svg" />
    <img alt="github-snake" src="resources/banners/light.svg" />
</picture>

## Introduction
Sakura or formerly known as Mangadex Reader is a desktop application for reading mangas, manhwas and manhuas from [Mangadex](https://mangadex.org). It can also be used to track different mangas across different sites. 

The UI for the application won't be looking that good for the time being as this is a project that I'm currently working on in my freetime when I don't have school but I will try to improve it bit by bit overtime.

### Contributors
<p align="center">
  <a href="https://github.com/phoeenix05">
    <img src="https://github.com/phoeenix05.png" alt="Alt Text" width="64" height="64" style="border-radius: .2rem;">
  </a>
</p>

![Static Badge](https://img.shields.io/badge/NuxtJS-%2300DC82?style=for-the-badge&logo=nuxtjs3&link=https%3A%2F%2Fnuxt.com%2F)
![Static Badge](https://img.shields.io/badge/tauri-1C1C1C?style=for-the-badge&logo=tauri&logoColor=FFD000&link=https%3A%2F%2Ftauri.app)

> ![Static Badge](https://img.shields.io/badge/surrealdb-1C1C1C?style=for-the-badge&logo=surrealdb&logoColor=D401CD&link=https%3A%2F%2Fsurrealdb.com%2F)

## Installation
I'll try to get A workflow for bundling the app working fot the beta release but for now you'll need to build the project yourself. You can check the [releases](https://github.com/Phoeenix05/sakura/releases) for application bundle for your platform.

## Development

#### Clone the repository
```zsh
git clone https://github.com/phoeenix05/sakura
```

#### Installing dependencies
```zsh
pnpm install
yarn install
npm install
```

#### Running the project
```zsh 
pnpm dev
yarn dev
npm run dev
```

#### Building the project
```zsh 
pnpm tauri build
yarn tauri build
npx tauri build
# or (not recommended as installing it is slow)
cargo tauri build
```

## Credits
Credits to [Mangadex](https://mangadex.org) for providing the ability to read chapters.

## Contribution guidelines
None. If you want to contribute just fork the repository make your changes and create a pull request. If there are things I would like to change in that pull request then I'll change them and merge the PR.

## Licensing
I am completely free and licensed under the [MIT license](https://github.com/Phoeenix05/sakura/blob/main/LICENSE). But if you like, you can feed me with a star on [GitHub](https://github.com/Phoeenix05/sakura).

{ pkgs ? import <nixpkgs> {} }: pkgs.mkShell {
    nativeBuildInputs = with pkgs.buildPackages; [
        cargo
        nodePackages_latest.pnpm
    ];
}
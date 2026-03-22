{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);

        rust = pkgs.rust-bin.stable."1.88.0".default;

        runtimeDeps = with pkgs; [
        ];
        buildDeps = with pkgs; [
          rustPlatform.bindgenHook
          rust
        ];
        devDeps = with pkgs; [
        ];
      in {
        devShells.default = with pkgs;
          mkShell {
            buildInputs = runtimeDeps;
            nativeBuildInputs = buildDeps ++ devDeps;

            LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath runtimeDeps}";
          };
      }
    );
}

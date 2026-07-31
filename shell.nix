{ self, pkgs, ... }:
let
  rustToolchain = pkgs.rust-bin.stable.latest.default.override {
    extensions = [
      "rust-src"
      "rust-analyzer"
    ];
  };
in
pkgs.mkShell {
  packages = with pkgs; [
    rustToolchain
    cargo-watch
    libiconv
    # Nix
    alejandra
    nixd
    deadnix
    statix
    self.formatter.${pkgs.system}
  ];
}

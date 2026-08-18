{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    clippy
    pkg-config
    gcc
    git
    python3
  ];

  shellHook = ''
    echo "🚀 Gooney-emu Development Environment Loaded!"
    echo "Rust version: $(rustc --version)"
    echo "Cargo version: $(cargo --version)"
    echo "Python version: $(python3 --version)"
  '';
}

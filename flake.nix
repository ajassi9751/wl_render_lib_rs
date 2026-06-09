{
  description = "Flake to build wl_render_lib_rs";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
    pkgs64 = import nixpkgs { system = "x86_64-linux"; };
  in
  {
    packages.x86_64-linux.default = pkgs64.rustPlatform.buildRustPackage rec {
      pname = "wl_render_lib_rs";
      version = "0.0.0";
      src = ./.;
      cargoLock.lockFile = ./Cargo.lock;
    };
    devShells.x86_64-linux.default = pkgs64.mkShell {
      buildInputs = with pkgs64; [
        curl
        unzip
        cargo
      ];
      shellHook = ''
        # Get test images
        if [ ! -d "qoi_test_images" ]; then
          curl -o img.zip https://qoiformat.org/qoi_test_images.zip
          unzip img.zip
          rm img.zip
        fi
      '';
    };
  };
}

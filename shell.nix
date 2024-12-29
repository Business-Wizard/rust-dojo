let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-24.05";
  pkgs = import nixpkgs { config = {allowUnfree=true;}; overlays = []; };
in

let
  shell_packages = with pkgs; [
    pre-commit
      openssl
    uv
  ];
in

pkgs.mkShell {
  buildInputs = shell_packages;
}

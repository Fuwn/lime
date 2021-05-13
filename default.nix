{ sources ? import ./nix/sources.nix, pkgs ? import sources.nixpkgs { } }:
with pkgs;

let
  rust = pkgs.callPackage ./nix/rust.nix { };

  srcNoTarget = dir:
    builtins.filterSource
      (path: type: type != "directory" || builtins.baseNameOf path != "target")
      dir;

  naersk = pkgs.callPackage sources.naersk {
    rustc = rust;
    cargo = rust;
  };
  dhallpkgs = import sources.easy-dhall-nix { inherit pkgs; };
  src = srcNoTarget ./.;

  lime = naersk.buildPackage {
    inherit src;
    doCheck = true;
    buildInputs = [ ];
    remapPathPrefix = true;
  };

  config = stdenv.mkDerivation {
    pname = "lime-config";
    version = "HEAD";
    buildInputs = [ dhallpkgs.dhall-simple ];

#    phases = "installPhase";
#
#    installPhase = ''
#    '';
  };

in pkgs.stdenv.mkDerivation {
  inherit (lime) name;
  inherit src;
  phases = "installPhase";

  installPhase = ''
    mkdir -p $out $out/bin

    cp -rf ${lime}/bin/lime $out/bin/lime
  '';
}

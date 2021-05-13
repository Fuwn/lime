{ system ? builtins.currentSystem }:

let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs { };
  callPackage = pkgs.lib.callPackageWith pkgs;
  lime = callPackage ./default.nix { };

  dockerImage = pkg:
    pkgs.dockerTools.buildImage {
      name = "fuwn/lime";
      tag = "latest";
      created = "now";

      fromImage = pkgs.dockerTools.pullImage {
        imageName = "alpine";
        imageDigest = "sha256:def822f9851ca422481ec6fee59a9966f12b351c62ccb9aca841526ffaa9f748";
        sha256 = "1z6fh6ry14m5cpcjfg88vn2m36garmgdagr4vfc3pm1z3kph639n";
        finalImageTag = "alpine";
        finalImageName = "3.13.5";
      };

      contents = [ pkg ];

      config = {
        WorkingDir = "/";
        Env = [ "DATABASE_URl=lime.sqlite3" ];
        ExposedPorts = {
          "8000/tcp" = { }; # API
        };
        EntryPoint = [ "/bin/lime" ];
      };
    };

in dockerImage lime

{ sources ? import ./sources.nix }:

let
  pkgs = import sources.nixpkgs { overlays = [ (import sources.nixpkgs-mozilla) ]; };
  channel = "nightly";
  date = "2021-05-12";
  targets = [ ];
  chan = pkgs.rustChannelOfTargets channel date targets;
in chan

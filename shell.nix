{ pkgs ? import <nixos-unstable> { } }:
pkgs.mkShell { packages = with pkgs; [ rust-analyzer pest-ide-tools bacon ]; }

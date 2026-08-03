{ pkgs, lib, config, inputs, ... }:

{
  packages = [
    pkgs.git
    pkgs.yamlfix
  ];
}

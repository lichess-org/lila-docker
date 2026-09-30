{ pkgs, lib, config, inputs, ... }:

let
  depotVersion = "2.102.7";

  # sha256 of each release tarball, from depot's checksums.txt:
  # https://github.com/depot/cli/releases/download/v${depotVersion}/depot_${depotVersion}_checksums.txt
  depotPlatforms = {
    "x86_64-linux" = {
      target = "linux_amd64";
      sha256 = "576ff8d378c8a74620b61d813513e5286f2059cde4596e3a31eccc2005a74669";
    };
    "aarch64-linux" = {
      target = "linux_arm64";
      sha256 = "ab455358d3c180a3c600c996de503727e9670a344b4a7fd0beb550f349cbbe3c";
    };
    "x86_64-darwin" = {
      target = "darwin_amd64";
      sha256 = "10887b646244158ea24aa12d153720990612ed9f8799ff47cdff1f374fd58ac8";
    };
    "aarch64-darwin" = {
      target = "darwin_arm64";
      sha256 = "ac136ac322b60a1f7c2220ee1a3bd69c3e8a9650b675ab2b2ebbdbf87ecb7eb7";
    };
  };
  depotPlatform = depotPlatforms.${pkgs.stdenv.hostPlatform.system}
    or (throw "depot-cli: unsupported platform ${pkgs.stdenv.hostPlatform.system}");

  depot-cli = pkgs.stdenv.mkDerivation {
    pname = "depot";
    version = depotVersion;

    src = pkgs.fetchurl {
      url = "https://github.com/depot/cli/releases/download/v${depotVersion}/depot_${depotVersion}_${depotPlatform.target}.tar.gz";
      sha256 = depotPlatform.sha256;
    };

    sourceRoot = ".";
    dontBuild = true;
    installPhase = ''
      mkdir -p $out/bin
      cp bin/depot $out/bin/depot
      chmod +x $out/bin/depot
    '';
  };
in
{
  env = {
    PASSWORD_STORE_DIR = "${config.env.DEVENV_ROOT}/pass";

    DEPOT_TOKEN = config.secretspec.secrets.DEPOT_TOKEN or "";
  };

  packages = [
    pkgs.git
    pkgs.bun
    pkgs.secretspec
    pkgs.yamlfix
    depot-cli
  ];

  dotenv.disableHint = true;

  scripts = {
    preview = {
      exec = ''
        secretspec run --provider keyring --profile previews -- bun run "${config.env.DEVENV_ROOT}/previews/preview.ts" "$@"
      '';
    };
  };
}

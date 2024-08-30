{ pkgs ? import <nixpkgs> 
}: {
  mkEnv = envOpts: pkgs.stdenv.mkDerivation {
    name = "vic-env";
    buildPhase = ''
    mkdir -p $out
    mkdir -p $out/nix-support
    echo -en ${builtins.toJSON envOpts} > $out/nix-support/vicEnv
    '';
  };
}

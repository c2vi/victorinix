{ pkgs ? import <nixpkgs>
, vicEnv ? import ./env.nix { inherit pkgs; }

}: rec {

  extra = rec {
    # the default thing to run with vic
    default = minimal;

    empty = vicEnv {};

    minimal = vicEnv {
    };
  };

  winePkgs = {
  };

}

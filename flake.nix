{
	nixConfig.bash-prompt = ''\033[22m\033[31mdev \033[01;34m\W\033[00m ❯❯❯ '';
  inputs = {
    # nixpkgs 35.11 still contains rust 1.73
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

 		flake-utils.url = "github:numtide/flake-utils";

    c2vi-config.url = "github:c2vi/nixos";
  };
  outputs = { self, flake-utils, nixpkgs, c2vi-config, ... }@inputs: 
  
  ####################################
  # outputs the same for every system
  {

  } //

  ####################################
  # system specific outpugs
  flake-utils.lib.eachDefaultSystem (system: let pkgs = nixpkgs.legacyPackages.${system}; in {

    packages.webfiles = pkgs.callPackage ./webfiles.nix {inherit inputs nixpkgs self c2vi-config; url = "http://c2vi.dev"; };
    packages.webrun = pkgs.callPackage ./webrun.nix {inherit inputs nixpkgs self; url = "http://c2vi.dev"; };
      
    devShells.default = pkgs.mkShell {
      buildInputs = with pkgs; [ libelf cargo rustc ];
    };
  });
}


{
	nixConfig.bash-prompt = ''\033[22m\033[31mdev \033[01;34m\W\033[00m ❯❯❯ '';
  inputs = {
    # nixpkgs 35.11 still contains rust 1.73
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";


 		flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { flake-utils, nixpkgs, ... }@inputs: flake-utils.lib.eachDefaultSystem (system: let pkgs = nixpkgs.legacyPackages.${system}; in {

      
    #packages.default =
    devShells.default = pkgs.mkShell {
      buildInputs = with pkgs; [ libelf cargo rustc ];
    };
  });
}


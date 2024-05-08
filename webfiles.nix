{ stdenv
, pkgs
, url
, nixpkgs
, self
, system
, c2vi-config
, ... }:
let
  pkgsStatic = (import nixpkgs { inherit system; overlays = [ c2vi-config.overlays.static ]; }).pkgsStatic;

  closure-x86_64-linux = pkgs.buildPackages.closureInfo { rootPaths = [ pkgsStatic.proot ]; };

  #victorinix-l = let pkgs = nixpkgs.legacyPackages.x86_64-linux; in pkgs.rustPlatform.buildRustPackage rec {
  victorinix-l = let l-pkgs = if pkgs.system == "x86_64-linux" then nixpkgs.legacyPackages.x86_64-linux.pkgsStatic else pkgs.pkgsCross.x86_64-linux.pkgsStatic; in l-pkgs.rustPlatform.buildRustPackage rec {
    name = "victorinix-l";
    buildInputs = with l-pkgs; [ libelf ];
    #RUSTFLAGS = "-C target-feature=+crt-static";
    src = self;
    #cargoSha256 = "sha256-T9Zb8wtL4cJi23u5IXxJ2qb44IzZO6LO6sXOqTj1S0Q=";
    cargoSha256 = "sha256-TaQWt3sh/TrWmNdvEGH0mIoQp0kOO2TUlVRfyoMDWZI=";
  };

  #victorinix-la = let pkgs = nixpkgs.legacyPackages.aarch64-linux; in pkgs.rustPlatform.buildRustPackage rec {
  victorinix-la = let la-pkgs = if pkgs.system == "aarch64-linux" then pkgs.pkgsStatic else pkgs.pkgsCross.aarch64-multiplatform.pkgsStatic; in la-pkgs.rustPlatform.buildRustPackage rec {
    name = "victorinix-la";
    buildInputs = with la-pkgs; [ libelf ];
    #RUSTFLAGS = "-C target-feature=+crt-static";
    src = self;
    cargoSha256 = "sha256-T9Zb8wtL4cJi23u5IXxJ2qb44IzZO6LO6sXOqTj1S0Q=";
  };

  victorinix-s = pkgs.writeTextFile {
    name = "victorinix-s";
    executable = true;

    text = ''
      #!/bin/sh
      # This is a quick script that downloads the correct victorinix binary to ./vic
      # Programms needed in $PATH: 
      # - sh (at /bin/sh)
      # - uname
      # - chmod
      # - wget or curl or python with urllib
      # - /dev/null

      ##########################
      # check for needed things
      if [ -f "/dev/null" ]; then echo "/dev/null exists"; else echo /dev/null does not exist; exit 1; fi
      if command -v uname >/dev/null; then echo uname found; else echo uname not found; exit 1; fi
      if command -v chmod >/dev/null; then echo chmod found; else echo chmod not found; exit 1; fi

      ##########################
      # determine right executable
      arch=$(uname -m)
      kernel=$(uname -s)
      exepath=""
      if [[ "$arch" == "x86_64" ]] && [[ "$kernel" == "Linux" ]]; then
          exepath="l/vic"
      elif [[ "$arch" == "aarch64" ]] && [[ "$kernel" == "Linux" ]]; then
          exepath="la/vic"
      else
        echo "system (kernel: $kernel, arch: $arch) not supported"
        exit 1
      fi


      ##########################
      # get executable
      echo downloading victorinix binary at ${url}/$exepath
      function download_with_python(){
        python=$1
        $python -c '

      # i hate you python with your indents!!!
      from urllib.request import urlopen
      import sys

      with urlopen("${url}" + "/" + sys.argv[1]) as response:
        body = response.read()

      f = open("./vic", "wb")
      f.write(body)
      f.close()
        ' $exepath
      }
      if command -v wget >/dev/null; then
        wget ${url}/$exepath

      elif command -v curl >/dev/null; then
        echo wget not found, trying curl
        curl ${url}/$exepath -o ./vic

      elif command -v python >/dev/null; then
        echo wget, curl not found, trying python
        download_with_python python

      elif command -v python3 >/dev/null; then
        echo wget, curl, python not found, trying python3
        download_with_python python3

      elif command -v pyton2 >/dev/null; then
        echo wget, curl, python, python3 not found, trying python2
        download_with_python python2

      else
        echo "wget, curl, python, python2, python3 not found ... out of ways to download the victorinix binary at ${url}$exepath"
        exit 1
      fi

      ##########################
      # make it executable
      chmod +x ./vic
    '';
  };

in

stdenv.mkDerivation rec {
  name = "victorinix-webfiles";
  dontUnpack = true;


  buildCommand = ''
    mkdir -p $out
    mkdir -p $out/l
    mkdir -p $out/la
    cp ${victorinix-s} $out/s
    cp ${victorinix-l}/bin/victorinix $out/l/vic
    cp ${victorinix-la}/bin/victorinix $out/la/vic

    mkdir -p $out/tars
    
    # make tars
    tar -c -f $out/tars/x86_64-linux.tar.gz -C / --files-from ${closure-x86_64-linux}/store-paths

  '';

    #${pkgs.gnutar}/bin/tar -C ./nix-store -czf $out/tars/x86_64-linux.tar.gz .

	nativeBuildInputs = [
	];
}


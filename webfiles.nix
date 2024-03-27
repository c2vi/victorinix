{ stdenv
, pkgs
, url
, nixpkgs
, self
, ... }:

stdenv.mkDerivation rec {
  name = "victorinix-webfiles";
  dontUnpack = true;

  #victorinix-l = let pkgs = nixpkgs.legacyPackages.x86_64-linux; in pkgs.rustPlatform.buildRustPackage rec {
  victorinix-l = let pkgs = nixpkgs.legacyPackages.x86_64-linux.pkgsMusl; in pkgs.rustPlatform.buildRustPackage rec {
    name = "victorinix-l";
    buildInputs = with pkgs; [ libelf ];
    src = self;
    #cargoSha256 = "sha256-T9Zb8wtL4cJi23u5IXxJ2qb44IzZO6LO6sXOqTj1S0Q=";
    cargoSha256 = "sha256-TaQWt3sh/TrWmNdvEGH0mIoQp0kOO2TUlVRfyoMDWZI=";
  };

  #victorinix-la = let pkgs = nixpkgs.legacyPackages.aarch64-linux; in pkgs.rustPlatform.buildRustPackage rec {
  victorinix-la = let pkgs = nixpkgs.legacyPackages.x86_64-linux.pkgsCross.aarch64-multiplatform; in pkgs.rustPlatform.buildRustPackage rec {
    name = "victorinix-la";
    buildInputs = with pkgs; [ libelf ];
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
      if command -v uname >/dev/null; then echo uname found; else echo uname not found; exit 1; fi
      if command -v chmod >/dev/null; then echo chmod found; else echo chmod not found; exit 1; fi


      ##########################
      # determine right executable
      arch=$(uname -m)
      kernel=$(uname -s)
      exepath=""
      if [[ "$arch" == "x86_64" ]] && [[ "$kernel" == "Linux" ]]; then
          exepath="l"
      elif [[ "$arch" == "aarch64" ]] && [[ "$kernel" == "Linux" ]]; then
          exepath="la"
      else
        echo system (kernel: $kernel, arch: $arch) not supported
        exit 1
      fi


      ##########################
      # get executable
      function download_with_python(){
        python=$1
        $python -c '
          import requests
          r = requests.get(${url}, allow_redirects=True)
          f = open("./vic", "wb")
          f.write(r.content)
          f.close()
        '
      }
      if command -v wget >/dev/null; then
        wget ${url}/$exepath
      else echo wget not found, trying curl
      fi

      if command -v curl >/dev/null; then
        curl ${url}/$exepath -o ./vic
      else echo curl not found, trying python
      fi

      if command -v pyton >/dev/null; then
        download_with_python python
      else echo curl not found, trying python
      fi

      if command -v python2 >/dev/null; then
        download_with_python python2
      else echo curl not found, trying python
      fi

      if command -v pyton3 >/dev/null; then
        download_with_python python3
      else echo curl not found, trying python
      fi

      ##########################
      # make it executable
      chmod +x ./vic
    '';
  };

  buildPhase = ''
    mkdir -p $out
    cp ${victorinix-s} $out/s
    cp ${victorinix-l}/bin/victorinix $out/l
    cp ${victorinix-la}/bin/victorinix $out/la
  '';

	nativeBuildInputs = [
	];
}


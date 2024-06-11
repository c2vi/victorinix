{ stdenv
, pkgs
, url
, nixpkgs
, self
, system
, c2vi-config
, vicConfig
, getTarballClosure
, getVicorinix
, ... }:
let
  closure-x86_64-linux = getTarballClosure pkgs "x86_64-linux";

  closure-aarch64-linux = getTarballClosure pkgs "aarch64-linux";

  #victorinix-l = let pkgs = nixpkgs.legacyPackages.x86_64-linux; in pkgs.rustPlatform.buildRustPackage rec {
  victorinix-l = getVicorinix pkgs "x86_64-linux" "l" "sha256-0kAb+sieN+Ipnr8E3CS3oy+9+4qvUQU3rXrhpJyGTIM=";
  victorinix-la = getVicorinix pkgs "aarch64-multiplatform" "la" "sha256-eB/+tcI5+pWSMq2fIKI3qPcuRKOg0r1C3/wm999G8CE=";

  victorinix-s = pkgs.writeTextFile {
    name = "victorinix-s";
    executable = true;

    text = ''
      #!/bin/sh
      # This is a quick script that downloads the correct victorinix binary to ./vic
      # Programms needed in $PATH or /bin:
      # - sh (at /bin/sh)
      # - uname
      # - chmod
      # - wget or curl or python with urllib

      ##########################
      # check for needed things

      # add /bin to path, so that even if there is no path specified we can run if /bin/uname and /bin/chmod exist
      PATH=$PATH:/bin
      dev_null_replacement=$(command -v uname)
      if [[ "@?" == "0" ]]; then echo uname found; else echo uname not found; exit 1; fi
      dev_null_replacement=$(command -v chmod)
      if [[ "@?" == "0" ]]; then echo chmod found; else echo chmod not found; exit 1; fi


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
      dev_null_replacement=$(command -v wget)
      if [[ "@?" == "0" ]]; then
        wget ${url}/$exepath

      dev_null_replacement=$(command -v curl)
      elif [[ "@?" == "0" ]]; then
        echo wget not found, trying curl
        curl ${url}/$exepath -o ./vic

      dev_null_replacement=$(command -v python)
      elif [[ "@?" == "0" ]]; then
        echo wget, curl not found, trying python
        download_with_python python

      dev_null_replacement=$(command -v python3)
      elif [[ "@?" == "0" ]]; then
        echo wget, curl, python not found, trying python3
        download_with_python python3

      dev_null_replacement=$(command -v python2)
      elif [[ "@?" == "0" ]]; then
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
webfilesBuildPhase = closure: ''

  mkdir -p tar-tmp
  mkdir -p tar-tmp/nix

  while read path
  do
    mkdir -p tar-tmp$path
    cp -r --no-preserve=mode,ownership $path tar-tmp$path
  done < ${closure.info}/store-paths
  cp ${closure.proot}/bin/proot tar-tmp/nix/proot
  tar -z -c -f $out/tars/x86_64-linux.tar.gz -C tar-tmp --mode="a+rwx" .

  rm -rf tar-tmp
'';

in

stdenv.mkDerivation {
  name = "victorinix-webfiles";
  dontUnpack = true;

  # so that /bin/sh does not get patched to a nix store path in victorinix-s
  dontPatchShebangs = true;

  buildPhase = ''
    mkdir -p $out
    mkdir -p $out/l
    mkdir -p $out/la

    cp ${victorinix-s} $out/s

    cp ${victorinix-l}/bin/victorinix $out/l/vic
    cp ${victorinix-la}/bin/victorinix $out/la/vic

    mkdir -p $out/tars
    
    # make tars

  '' + webfilesBuildPhase closure-x86_64-linux + webfilesBuildPhase closure-aarch64-linux;

    #${pkgs.gnutar}/bin/tar -C ./nix-store -czf $out/tars/x86_64-linux.tar.gz .

	nativeBuildInputs = [
	];
}


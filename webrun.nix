{ stdenv
, ... }:

stdenv.mkDerivation rec {
  name = "victorinix-webhost";

  dontUnpack = true;

  buildPhase = ''
    echo hiiiiiiiiiiii > $out/works
  '';

	nativeBuildInputs = [
	];
}

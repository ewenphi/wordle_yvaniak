{
  pkgs ? import <nixpkgs> { },
  crane,
}:
let
  craneLib = crane.mkLib pkgs;

  commonArgs = {
    src = craneLib.cleanCargoSource ./..;
    strictDeps = true;

    buildInputs = [
    ];
  };

  cargoArtifacts = craneLib.buildDepsOnly commonArgs;

  wordle_yvaniak = craneLib.buildPackage (
    commonArgs
    // {
      inherit cargoArtifacts;
      useNextest = true;
    }
  );

  wordle_yvaniak-docs = craneLib.cargoDoc (
    commonArgs
    // {
      inherit cargoArtifacts;
    }
  );
in
{
  default = wordle_yvaniak;
  docs = wordle_yvaniak-docs;
}

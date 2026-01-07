{
  description = "orb-internal flake";
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    orb-software.url = "github:worldcoin/orb-software";
  };

  outputs = { self, flake-utils, orb-software }:
    # This helper function is used to more easily abstract
    # over the host platform.
    # See https://github.com/numtide/flake-utils#eachdefaultsystem--system---attrs
    flake-utils.lib.eachDefaultSystem (system:
      # See https://nixos.wiki/wiki/Flakes#Output_schema
      {
        devShells = orb-software.devShells.${system};
        formatter = orb-software.formatter.${system};
      }
    );
}

{
  description = "orb-mcu-messaging flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    # Provides eachDefaultSystem and other utility functions
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:

    # This helper function is used to more easily abstract
    # over the host platform.
    # See https://github.com/numtide/flake-utils#eachdefaultsystem--system---attrs
    utils.lib.eachDefaultSystem (system:
      let
        p = {
          # The platform that you are running nix on and building from
          native = nixpkgs.legacyPackages.${system};
        };
      in
      {
        # Everything in here becomes your shell (nix develop)
        devShells.default = p.native.mkShell {
          # Nix makes the following list of dependencies available to the development
          # environment.
          buildInputs = (with p.native; [
            # Needed for cargo zigbuild
            zig
            cargo-zigbuild

            # Used by various rust build scripts to find system libs
            # Note that this is the unwrapped version of pkg-config. By default,
            # nix wraps pkg-config with a script that replaces the PKG_CONFIG_PATH
            # with the proper settings for cross compilation. We already set these
            # env variables ourselves and don't want nix overwriting them, so we
            # use the unwrapped version.
            pkg-config-unwrapped

            # Developer tools
            cargo-deny
            cargo-expand
            cargo-binutils
            protobuf
            nixpkgs-fmt
            pre-commit

            # This is missing on mac m1 nix, for some reason.
            # see https://stackoverflow.com/a/69732679
            libiconv
          ]);

          # The following sets up environment variables for the shell. These are used
          # by the build.rs build scripts of the rust crates.
          shellHook = ''
            # Env vars here
          '';
        };
        # Lets you type `nix fmt` to format the flake.
        formatter = p.native.nixpkgs-fmt;
      }

    );

}

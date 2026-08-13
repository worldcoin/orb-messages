{
  description = "orb-messages flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    # Keep cargo-deny new enough to parse CVSS 4.0 advisories without updating
    # the rest of the development environment.
    nixpkgs-cargo-deny.url = "github:NixOS/nixpkgs/7a1a64774a5fd0b0cd39ac95d0e170ace8b266a0";
    # Provides eachDefaultSystem and other utility functions
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, nixpkgs-cargo-deny, utils }:

    # This helper function is used to more easily abstract
    # over the host platform.
    # See https://github.com/numtide/flake-utils#eachdefaultsystem--system---attrs
    utils.lib.eachDefaultSystem (system:
      let
        p = {
          # The platform that you are running nix on and building from
          native = nixpkgs.legacyPackages.${system};
          cargoDeny = nixpkgs-cargo-deny.legacyPackages.${system}.cargo-deny;
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
            clang-tools
            cargo-binutils
            p.cargoDeny
            cargo-expand
            nixpkgs-fmt
            pre-commit
            protobuf

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

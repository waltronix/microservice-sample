{
  description = "Library microservices";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.mkLib pkgs;

        commonArgs = {
          src = pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = path: type:
              (craneLib.filterCargoSources path type)
              || (pkgs.lib.hasSuffix ".sql" path);
            name = "source";
          };
          strictDeps = true;
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.libpq pkgs.openssl ]
            ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [ pkgs.libiconv ];
          PKG_CONFIG_PATH = "${pkgs.libpq}/lib/pkgconfig:${pkgs.openssl.dev}/lib/pkgconfig";
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        mkService = name: craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          cargoExtraArgs = "--bin ${name}";
          doCheck = false;
        });

        mkImage = name: bin: pkgs.dockerTools.buildLayeredImage {
          inherit name;
          tag = "latest";
          contents = [ bin pkgs.cacert pkgs.libpq ];
          config.Cmd = [ "/bin/${name}" ];
        };

        userService   = mkService "user-service";
        bookService   = mkService "book-service";
        reviewService = mkService "review-service";
      in
      {
        packages = {
          user-service         = userService;
          user-service-image   = mkImage "user-service" userService;
          book-service         = bookService;
          book-service-image   = mkImage "book-service" bookService;
          review-service       = reviewService;
          review-service-image = mkImage "review-service" reviewService;
        };

        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.rustc
            pkgs.cargo
            pkgs.rustfmt
            pkgs.clippy
            pkgs.pkg-config
            pkgs.libpq
            pkgs.diesel-cli
            pkgs.cargo-watch
            pkgs.cargo-nextest
          ];
          PKG_CONFIG_PATH = "${pkgs.libpq}/lib/pkgconfig:${pkgs.openssl.dev}/lib/pkgconfig";
          LIBPQ_DIR = "${pkgs.libpq}";
          shellHook = "echo 'microservice-sample dev shell'";
        };
      });
}

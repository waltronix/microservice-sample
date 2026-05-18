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

        # Build deps for all workspace members except the e2e test crate.
        # authz-bdd has dev-only deps (testcontainers) that don't belong in
        # service images; --exclude keeps it out of the dep compilation without
        # removing it from the workspace or the lock file.
        cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
          pname = "library-services";
          version = "0.0.1";
          cargoExtraArgs = "--exclude authz-bdd --workspace";
        });

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
            pkgs.openfga-cli
            pkgs.open-policy-agent
            # docker-compose CLI (v2) — the e2e test harness calls it directly.
            # Works against the rootless Podman socket via DOCKER_HOST.
            pkgs.docker-compose
          ];
          PKG_CONFIG_PATH = "${pkgs.libpq}/lib/pkgconfig:${pkgs.openssl.dev}/lib/pkgconfig";
          LIBPQ_DIR = "${pkgs.libpq}";
          # Point testcontainers (and any other Docker-compatible tooling) at the
          # rootless Podman socket. Evaluated at shell startup so $UID is correct.
          shellHook = ''
            export DOCKER_HOST="unix:///run/user/$UID/podman/podman.sock"
            echo 'microservice-sample dev shell'
          '';
        };
      });
}

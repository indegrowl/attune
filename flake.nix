{
  description = "attune daemon";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    let
      nixosModule = { config, lib, pkgs, ... }:
        let
          cfg = config.services.attune;
        in
        {
          options.services.attune = {
            enable = lib.mkEnableOption "attune daemon";

            package = lib.mkPackageOption pkgs "attune" {
              default = [ "attune" ];
              extraDescription = "The attune package to use.";
            } // { default = self.packages.${pkgs.system}.default; };

            user = lib.mkOption {
              type = lib.types.str;
              default = "attune";
              description = "User to run attune as.";
            };

            group = lib.mkOption {
              type = lib.types.str;
              default = "attune";
              description = "Group to run attune as.";
            };
          };

          config = lib.mkIf cfg.enable {
            users.users.${cfg.user} = {
              isSystemUser = true;
              group = cfg.group;
              description = "attune daemon user";
            };

            users.groups.${cfg.group} = { };

            systemd.services.attune = {
              description = "attune daemon";
              wantedBy = [ "multi-user.target" ];
              after = [ "network.target" ];

              serviceConfig = {
                ExecStart = "${cfg.package}/bin/attune";
                User = cfg.user;
                Group = cfg.group;
                Restart = "on-failure";
                RestartSec = "5s";

                # Hardening
                NoNewPrivileges = true;
                PrivateTmp = true;
                ProtectSystem = "strict";
                ProtectHome = true;
                CapabilityBoundingSet = "";
                SystemCallArchitectures = "native";
              };
            };
          };
        };
    in
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "attune";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain
            pkgs.cargo-watch
            pkgs.nixpkgs-fmt
          ];
        };
      }
    ) // {
      nixosModules.default = nixosModule;
    };
}

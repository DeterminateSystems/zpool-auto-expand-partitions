let
  compat = import ./default.nix;
  zpool_tool = compat.defaultPackage.${builtins.currentSystem};
  nixpkgs = compat.inputs.nixpkgs;
in
import "${nixpkgs}/nixos/tests/make-test-python.nix" ({ pkgs, ... }:
  {
    name = "multi-disk-zfs";
    nodes = {
      machine =
        { pkgs, ... }: {
          environment.systemPackages = [ pkgs.parted pkgs.cloud-utils ];
          boot.supportedFilesystems = [ "zfs" ];
          networking.hostId = "00000000";

          # nixos config ...
          virtualisation = {
            emptyDiskImages = [ 20480 20480 20480 20480 20480 20480 ];
          };
        };
    };

    testScript = { nodes, ... }:
      ''
        start_all()
        machine.wait_for_unit("default.target")
        print(machine.succeed('mount'))

        print(machine.succeed('parted --script /dev/vdb -- mklabel gpt'))
        print(machine.succeed('parted --script /dev/vdb -- mkpart primary 1M 70M'))

        print(machine.succeed('parted --script /dev/vdc -- mklabel gpt'))
        print(machine.succeed('parted --script /dev/vdc -- mkpart primary 1M 70M'))

        print(machine.succeed('zpool create tank mirror /dev/vdb1 /dev/vdc1 mirror /dev/vdd /dev/vde mirror /dev/vdf /dev/vdg'))
        print(machine.succeed('zpool list -v'))
      '';
  })
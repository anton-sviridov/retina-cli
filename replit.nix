{ pkgs }: {
    deps = [
        pkgs.sqlite
        pkgs.diesel-cli
        pkgs.openssl
        pkgs.pkg-config
    ];
}
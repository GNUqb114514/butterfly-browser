{
  description = "Butterfly Browser: A browser that forgets tabs";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    ...
  }: let
    system = "x86_64-linux";
  in {
    devShells."${system}".default = let
      pkgs = import nixpkgs {inherit system;};
    in
      pkgs.mkShell {
        packages = with pkgs; [
          gtk4
          glib
          pkg-config
          webkitgtk_6_0
          openssl
          glib-networking
          gst_all_1.gstreamer
          gst_all_1.gst-plugins-base
          gst_all_1.gst-plugins-good
          gst_all_1.gst-plugins-bad
          gst_all_1.gst-plugins-ugly
        ];
        GIO_MODULE_DIR = "${pkgs.glib-networking}/lib/gio/modules/";
      };
  };
}

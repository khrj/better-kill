{
	description = "A better kill command";

	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs";
		fenix = {
			url = "github:nix-community/fenix";
			inputs.nixpkgs.follows = "nixpkgs";
		};
	};

	outputs = { self, fenix, nixpkgs, ... }:
		let pkgs = nixpkgs.legacyPackages.x86_64-linux;
		in {
			devShell.x86_64-linux = pkgs.mkShell {
				buildInputs = with pkgs; [
					fenix.packages.x86_64-linux.stable.defaultToolchain
					fenix.packages.x86_64-linux.rust-analyzer
					fenix.packages.x86_64-linux.rust-analyzer-vscode-extension
				];
			};
		};
}

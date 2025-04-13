BINDGEN := bindgen
CARGO := cargo
JQ := jq
MESON := meson


all:
	$(CARGO) clean
	$(CARGO) build --release

	rm -f ~/dot.files/.config/waybar/scripts/cpugraph/cpugraph-rs
	cp target/release/cpugraph-rs ~/dot.files/.config/waybar/scripts/cpugraph/

	rm -f ~/dot.files/.config/waybar/scripts/memgraph/memgraph-rs
	cp target/release/memgraph-rs ~/dot.files/.config/waybar/scripts/memgraph/

	rm -f ~/dot.files/.config/waybar/scripts/netgraph/netgraph-rs
	cp target/release/netgraph-rs ~/dot.files/.config/waybar/scripts/netgraph/

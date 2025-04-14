
CARGO := cargo

default: all

all:
	$(CARGO) clean
	$(CARGO) build --release

	rm -f ~/dot.files/.config/waybar/scripts/cpugraph/cpugraph-rs
	cp target/release/cpugraph-rs ~/dot.files/.config/waybar/scripts/cpugraph/

	rm -f ~/dot.files/.config/waybar/scripts/memgraph/memgraph-rs
	cp target/release/memgraph-rs ~/dot.files/.config/waybar/scripts/memgraph/

	rm -f ~/dot.files/.config/waybar/scripts/netgraph/netgraph-rs
	cp target/release/netgraph-rs ~/dot.files/.config/waybar/scripts/netgraph/

	rm -f ~/dot.files/.config/waybar/scripts/tempgraph/tempgraph-rs
	cp target/release/tempgraph-rs ~/dot.files/.config/waybar/scripts/tempgraph/

	rm -f ~/dot.files/.config/waybar/scripts/updates/archupdates-rs
	cp target/release/archupdates-rs ~/dot.files/.config/waybar/scripts/updates/
.PHONY: all

all: serve

serve:
	trunk serve

build-css:
	stylance .

leptos-fmt:
	cargo fmt
	leptosfmt srcec
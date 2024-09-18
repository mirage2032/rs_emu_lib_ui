.PHONY: all

all: serve

serve:
	trunk serve

build-css:
	stylance . --output-dir .\static\css\

leptos-fmt:
	cargo fmt
	leptosfmt src
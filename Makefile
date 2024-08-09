.PHONY: all

all: serve

serve:
	trunk serve

build-css:
	stylance . --output-dir .\static\css\


build-fmt:
	leptosfmt src
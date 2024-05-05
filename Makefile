.PHONY: build, docs

build:
	turbo run build

docs:
	cd docs && pnpm run dev


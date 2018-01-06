.PHONY: Spacer.app
Spacer.app:
	cargo build --release
	cp assets/Spacer.icns Spacer.app/Contents/Resources/
	cp target/release/spacer Spacer.app/Contents/MacOS/
	touch $@

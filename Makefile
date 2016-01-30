# Save this file to ~/.multirust/Makefile
# to run, cd ~/.multirust; make toolchains/1.3.0/src
.PHONY: clean

rust.git:
	git clone https://github.com/rust-lang/rust.git --bare

toolchains/nightly/src: toolchains/master/src
	ln -sf $$(pwd)/toolchains/master/src $$(pwd)/$@

toolchains/%/src: rust.git
	(cd rust.git; git fetch)
	rm -rf ./toolchains/$*/tmp
	mkdir -p ./toolchains/$*/tmp
	git archive --remote rust.git $* ./src --format tar | tar x -C ./toolchains/$*/tmp
	[ -d ./toolchains/$*/tmp ]
	mv ./toolchains/$*/tmp/src $@
	rmdir ./toolchains/$*/tmp
	touch $@

clean:
	rm -rf ./toolchains/*/src

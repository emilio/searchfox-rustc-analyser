Generate JSON output for [searchfox](https://github.com/bill-mccloskey/mozsearch) based on the result of rustc's save-analysis.

Usage:

    $ cd path/to/some-rust-project
    $ RUSTFLAGS=-Zsave-analysis cargo build
    $ DEPS=`pwd`/target/debug/deps/save-analysis
    $ SRC=`pwd`/src
    $ cd path/to/searchfox-rustc-analyser
    $ mkdir output
    $ cargo run $SRC $DEPS output/

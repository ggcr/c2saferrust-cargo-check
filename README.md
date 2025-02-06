## C2SaferRust: Transforming C Projects into Safer Rust with NeuroSymbolic Techniques

This is the code for our [paper](https://arxiv.org/pdf/2501.14257).

### Setup

You need Python 3.11 with Pip, and [Rustup](https://rustup.rs/). If you want to re-generate our translations, you also need an [OpenAI API](https://platform.openai.com/docs/api-reference/authentication) key. Otherwise, if you'd simply like to compute metrics on our provided translations, you can skip this.

First, set the necessary environment variables.
```sh
export RUSTUP_HOME="<your_path_here>"
export RUSTFLAGS="-L $RUSTUP_HOME/toolchains/nightly-2022-08-08-x86_64-unknown-linux-gnu/lib"
export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$RUSTUP_HOME/toolchains/nightly-2022-08-08-x86_64-unknown-linux-gnu/lib"
echo 'OPENAI_API_KEY="<your_key_here>"' > models/.env
```

Install rustup toolchain dependencies.
```sh
rustup component add rust-src rustc-dev llvm-tools-preview
```

Install Python requirements.
```sh
pip install -r requirements.txt
```

Then, install the slicer.
```sh
cd slicer
cargo install --debug --locked --path . --force
cd ..
```

Then, install the metrics tool.
```sh
cd ../metrics
cargo install --debug --locked --path . --force
cd ..
```

Now, we are ready to run translation. You can skip this step if you only want to evaluate our provided translations.
```sh
python translate.py --code_dir coreutils/src/cat/rust --test_dir coreutils/tests/cat
```
In this manner, you can generate translation for all 7 utilities in `coreutils`, and all 10 utilities in `laertes_benchmarks`. Note that the Laertes Benchmarks don't come with end-to-end tests, so you can omit the `--test_dir` flag while generating those translations.

Finally, we evaluate the generated translations.
```sh
cd coreutils
python collect_results.py
```
This writes the results to a json file called `coreutils_results.json`. To generate Table 2 in our paper, from within the `coreutils` folder, run
```sh
python format_as_table.py
```

We can also do the same for the Laertes benchmarks to get Table 3.
```sh
cd ../laertes_benchmarks
python collect_results.py
python format_as_table.py
```

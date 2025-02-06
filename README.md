First, set the necessary environment variables.

```sh
export RUSTFLAGS="-L $RUSTUP_HOME/toolchains/nightly-2022-08-08-x86_64-unknown-linux-gnu/lib"
export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$RUSTUP_HOME/toolchains/nightly-2022-08-08-x86_64-unknown-linux-gnu/lib"
echo 'OPENAI_API_KEY="<your_key_here>"' > models/.env
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

Now we are ready to run translation.

```sh
python translate.py --code_dir coreutils/src/cat/rust --test_dir coreutils/tests/cat
```
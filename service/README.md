# RBP Service

Risk balanced portfolio backend

## Development

### Run

Run from the root project dir - as it requires python modules in current work dir.

```sh
cd ..
cargo run service
```

#### Recommended env vars

```sh
export ALLOWED_ORIGIN=http://rbp.local.katlex.com:8080 # for access with real mobile device through proxy (e.g. Charles)
export RUST_LOG=info
```

### Troubleshooting

#### Problem

`cargo run` fails with `dyld: Library not loaded: @rpath/libpython3.7m.dylib`

#### Solution

`export LD_LIBRARY_PATH=/opt/anaconda3/lib` <- choose your own path with python lib

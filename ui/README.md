# RBP UI

Risk balanced portfolio Rust/WebAssembly UI

## Development

### Pre-install

```sh
cargo install simple-http-server
npm install -g yarn
```

### Install

Node.js stuff

```sh
yarn
```

If you set `NODE_ENV=productions` yarn won't install `devDependencies` which are the only dependencies for this project so far.

You can workaround this problem with `yarn --production=false`.

### Build css

```sh
npx tailwindcss build styles.css -o pkg/styles.css
```

### Run

Live rebuild

```sh
cargo watch -s 'wasm-pack build --target web && rollup -c'
```

Serve

```sh
simple-http-server -p 8080 -i --nocache --try-file index.html
```

Go to http://localhost:8080/

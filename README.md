# Rust demo: Reduxrs and Wasm

![Demo](https://github.com/Schr3da/wasm-reduxrs-example/blob/master/demo.gif)

## How to build

- Install rust wasm32-unknown-unknown toolchain
  ```
  rustup target add wasm32-unknown-unknown
  ```

- Install wasm-pack
  ```
  cargo install wasm-pack
  ```

- Clone repository
  ```
  git clone https://github.com/Schr3da/wasm-reduxrs-example.git
  ```

- Change into wasm-reduxrs-example/app/wasm
  ```
  cd wasm-reduxrs-example/app/wasm
  ```

- Build project using wasm-pack
  ```
  wasm-pack build --target web
  ```
  
- Use a static http-server to host application
  ```
  http-server wasm-reduxrs-example/app/wasm/
  ```
  
- Use browser to access the index.html page e.g.
  ```
  http://127.0.0.1:8080/index.html
  ```

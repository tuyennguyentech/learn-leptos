# Leptos Developer Experience Improvements

There are a couple of things you can do to improve your experience of developing websites and apps with Leptos. You may want to take a few minutes and set up your environment to optimize your development experience, especially if you want to code along with the examples in this book.

## Set up `console_error_panic_hook`

By default, panics that happen while running your WASM code in the browser just throw an error in the browser with an unhelpful message like `Unreachable executed` and a stack trace that points into your WASM binary.

With `console_error_panic_hook`, you get an actual Rust stack trace that includes a line in your Rust source code.

It's very easy to set up:

Run cargo add `console_error_panic_hook` in your project
In your main function, add `console_error_panic_hook::set_once();`


## Editor Autocompletion inside `#[component]` and `#[server]`

Because of the nature of macros (they can expand from anything to anything, but only if the input is exactly correct at that instant) it can be hard for rust-analyzer to do proper autocompletion and other support.

If you run into issues using these macros in your editor, you can explicitly tell rust-analyzer to ignore certain proc macros. For the `#[server]` macro especially, which annotates function bodies but doesn't actually transform anything inside the body of your function, this can be really helpful.

Zed, in `settings.json`:

``` json
{
  -- Other Settings ...
  "lsp": {
    "rust-analyzer": {
      "procMacro": {
        "ignored": [
          // optional:
          // "component",
          "server"
        ]
      }
    }
  }
}
```

## Enable features in Rust-Analyzer for your Editor (optional)

By default, rust-analyzer will only run against the default features in your Rust project. Leptos uses different features to control compilation. For client side rendered projects, we use `csr` in different places, for server side rendered apps they can include `ssr` for server code and hydrate for code that we'll only run in the browser.

How to enable these features varies by your IDE, we've listed some common ones below. If your IDE is not listed, you can usually find the setting by searching for `rust-analyzer.cargo.features` or `rust-analyzer.cargo.allFeatures`.

Zed, in `settings.json`:

``` json
{
  -- Other Settings ...
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "cargo": {
          "allFeatures": true // Enable all features
        }
      }
    }
  }
}
```

## Set up `leptosfmt` (optional)

`leptosfmt` is a formatter for the Leptos `view!` macro (inside of which you'll typically write your UI code). Because the `view!` macro enables an 'RSX' (like JSX) style of writing your UI's, `cargo-fmt` has a harder time auto-formatting your code that's inside the `view!` macro. `leptosfmt` is a crate that solves your formatting issues and keeps your RSX-style UI code looking nice and tidy!

`leptosfmt` can be installed and used via the command line or from within your code editor:

First, install the tool with `cargo install leptosfmt`.

If you just want to use the default options from the command line, just run `leptosfmt ./**/*.rs` from the root of your project to format all the rust files using `leptosfmt`.

## Use `--cfg=erase_components` during development

Leptos 0.7 made a number of changes to the renderer that relied more heavily on the type system. For larger projects, this can lead to slower compile times. Most of the slowdown in compile times can be alleviated by using the custom configuration flag `--cfg=erase_components` during development. (This erases some of that type information to reduce the amount of work done and debug info emitted by the compiler, at the expense of additional binary size and runtime cost, so it’s best not to use it in release mode.)

As of cargo-leptos v0.2.40, this is automatically enabled for you in development mode. If you are using `trunk`, not using `cargo-lepto`s, or want to enable it for non-dev uses, you can set this easily in the command line (`RUSTFLAGS="--cfg erase_components" trunk serve` or `RUSTFLAGS="--cfg erase_components" cargo leptos watch`), or in your `.cargo/config.toml`:

``` toml
# use your own native target
[target.aarch64-apple-darwin]
rustflags = [
  "--cfg",
  "erase_components",
]

[target.wasm32-unknown-unknown]
rustflags = [
   "--cfg",
   "erase_components",
]
```

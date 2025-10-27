# Getting Started

There are two basic paths to getting started with Leptos:

1. **Client-side rendering (CSR) with [Trunk](https://trunkrs.dev/)** - a greate option if you want to make a snappy website with Leptos, or work with a pre-existing server or API. In CSR mode, Trunk compiles your Leptos app to WebAssembly (WASM) and runs it in the browser like a typical JavaScript single-page app (SPA). The advantages of Leptos CSR include faster build times and a quicker interative development cycle, as well as a simpler mental model and more options for deploying your app. CSR apps do come with some disadvantages: inita load times for your end users are slower compared to a server-side rendering approach, and the usual SEO challenges that come along with using a JS single-page app model apply to Leptos CSR apps as well. Also note that, under the hood, an auto-generated snippet of JS is used to load the Leptos WASM bundle, so JS must be enabled on the client device for your CSR app to display properly. As with all software engineering, there are trade-offs here you'll need to consider.

2. **Full-stack, server-side rendering (SSR) with `cargo-leptos`** - SSR is greate option for building CRUD-style websites and custom web apps if you want Rust powering both your frontend and backend. With the Leptos SSR option, you app is rendered to HTML on the server and sent down to the browser; then, WebAssembly is used to instrument the HTML, so your app becomes interactive - this process is called 'hdration'. On the server side, Leptos SSR apps integrate closely with your choice of either [Actix-web](https://docs.rs/leptos_actix/latest/leptos_actix/) or [Axum](https://docs.rs/leptos_axum/latest/leptos_axum/) server libraries, so you can leverage those communities's crates to help build out your Leptos server. The advantages of taking the SSR route with Leptos include helping you get the best inital load times and optimal SEO scores for your web app. SSR apps can also dramatically simplify working across the server/client boundary via a Leptos feature called "server functions", which lets you transparently call functions on the server from your client code (more on this feature later). Full-stack SSR isn't all rainbows and butterflies, though - disadvantages include a slower developer iteration loop (because you need to recompile both the server and client when making Rust code changes), as well as some added complexity that comes along with hydration.

By the end of the book, you should have a good idea of which trade-offs to make and which route to take - CSR or SSR - depending on your project's requirements.

In Part 1 of this book, we'll start with client-side rendering Leptos sites and building reactive UIs using `Trunk` to serve our JS and WASM bundle to the browser.

We’ll introduce `cargo-leptos` in Part 2 of this book, which is all about working with the full power of Leptos in its full-stack, SSR mode.

## Hello World! Getting Set up for Leptos CSR development

Install "Trunk" tool for running Leptos CSR sites:

``` bash
cargo install trunk
```

And then create a basic Rust project:

``` bash
cargo init learn-leptos
```

`cd` into your new `learn-leptos` project and add `leptos` as a dependency:

``` bash
cargo add leptos --features=csr
```

Make sure you've added the `wasm32-unknown-unknown` target so that Rust can compile your code to WebAssembly to run in the browser:

``` bash
rustup target add wasm32-unknown-unknown
```

Create a simple `index.html` in the root of the `learn-leptos`-tutorial` directory:

``` html
<!DOCTYPE html>
<html>
  <head></head>
  <body></body>
</html>
```

And add a simple `“Hello, world!”` to your `main.rs`:

``` rust
use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(|| view! { <p>"Hello, world!"</p> })
}
```

Your directory structure should now look something like this:

``` bash
learn-leptos
├── src
│   └── main.rs
├── Cargo.toml
├── index.html
```

Now run `trunk serve --open` from the root of the leptos-tutorial directory. Trunk should automatically compile your app and open it in your default browser. If you make edits to `main.rs`, Trunk will recompile your source code and live-reload the page.

Welcome to the world of UI development with Rust and WebAssembly (WASM), powered by Leptos and Trunk!

Now before we get started building your first real applications with Leptos, there are a couple of things you might want to know to help make your experience with Leptos just a little bit easier.

Percy [![Build Status](https://travis-ci.org/chinedufn/percy.svg?branch=master)](https://travis-ci.org/chinedufn/percy) [![docs](https://docs.rs/percy/badge.svg)](https://docs.rs/percy)
===============

> A modular toolkit for building isomorphic web apps with Rust + WebAssembly

_The tools:_

[virtual-dom-rs](https://docs.rs/virtual_dom_rs)

[inline-stylesheets] <- WIP

## Initial Background / Motivation

I started using Rust in January 2018 and quickly got to the stage of "I REALLY want to use this for everything, even if it isn't the best tool for the job."

I need to make a website for a game that I'm working on, but the Rust ecosystem for frontend web apps with server side rendering is still very immature.

So I started working on a standalone virtual-dom implementation that could render to an HTML string on the server side and to a DOM element in the browser.

But then I realized that I wanted something similar to [sheetify](https://github.com/stackcss/sheetify).. And probably a couple other base web dev primitives too..

So I decided to make a cargo workspace with the tools that I needed to build isomorphic web apps in Rust. And here we are!

## Getting Started

TODO...

link to mdbook...

## Running the example isomorphic web app locally

```
git clone https://github.com/chinedufn/percy
cd percy
./examples/isomorphic/start.sh
```

Now visit `http://127.0.0.1:7878` !

## Contributing

Please open issues / PRs explaining your intended use case and let's see if we should or shouldn't make `percy` support it!

Also feel free to open issues and PRs with any questions / thoughts that you have!

## To test

TODO... make this a script so that we can run the JSDOM tests also

```sh
cargo test --all
```

## See Also

- [virtual-dom](https://github.com/Matt-Esch/virtual-dom) - a JavaScript virtual-dom implementation that I took inspiration from.

- [How to write your own Virtual DOM](https://medium.com/@deathmood/how-to-write-your-own-virtual-dom-ee74acc13060) - helped me better understand how a virtual-dom works.

- [Sheetify](https://github.com/stackcss/sheetify) inspired the css! macro

## License

MIT

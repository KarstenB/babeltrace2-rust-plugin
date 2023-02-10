This library is a rust wrapper for libbabeltrace2, which is used to develop plugins for [Babeltrace](https://babeltrace.org).

It essentially consists of 2 layers, the code generated from bindgen in mod `bt2::binding`, and an object oriented wrapper in mod `bt2`. The object oriented wrapper is automatically generated with the `code_gen.rs` file. If you want to regenerate the `bt2` module, you can run `cargo test`.

In order to use this library, you should have babeltrace and its library installed. For debian this can be done with:
```bash
apt install Babeltrace2 libbabeltrace2-dev
```

## Examples
There is a rust-translated version of the epitome example at [babeltrace2-rust-plugin-example-epitome](https://github.com/KarstenB/babeltrace2-rust-plugin-example-epitome).
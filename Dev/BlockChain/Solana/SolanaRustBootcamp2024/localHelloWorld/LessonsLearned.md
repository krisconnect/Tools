# Just remember!
### 2 hours of debugging can save you 5 minutes of reading the manuals!

##### Error 1
```error: package `borsh v1.4.0` cannot be built because it requires rustc 1.67.0 or newer, while the currently active rustc version is 1.62.0-dev```\
Tried many things but in the end just deleted everything remotely resembling solana and followed the official documentation...
##### Error 2
Do not forget to add ```[lib] crate-type = ["cdylib", "lib"] ``` to Cargo.toml file when dealing with lib...
##### Error 3
Weird error with the solana-sdk...
```
error: target is not supported, for more information see: https://docs.rs/getrandom/#unsupported-targets
   --> src/lib.rs:267:9
    |
267 | /         compile_error!("\
268 | |             target is not supported, for more information see: \
269 | |             https://docs.rs/getrandom/#unsupported-targets\
270 | |         ");
    | |__________^

   Compiling memchr v2.7.2
error[E0433]: failed to resolve: use of undeclared crate or module `imp`
   --> src/lib.rs:291:5
    |
291 |     imp::getrandom_inner(dest)
    |     ^^^ use of undeclared crate or module `imp`
```
Just like...get rid of it for now?...Stick to exact Cargo.toml
```
[package]
name = "program"
version = "0.1.0"
edition = "2021"

[dependencies]
solana-program = "1.18.9"

[lib]
crate-type = ["cdylib", "lib"] 
```
##### Error 4
When dealing with client programs maybe don't forget to ```npm install -g ts-node``` ...

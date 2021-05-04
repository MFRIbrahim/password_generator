# password_generator

A simple and safe CLI password generator.This tool uses the preferred random number source of the operating system through the [rand crate](https://github.com/rust-random/rand) and the [getrandom crate](https://github.com/rust-random/getrandom). This is a form of "true" randomness generated by hardware and should thus be very safe. 

# Usage
To use this tool build it with cargo. Simply [install Rust](https://www.rust-lang.org/tools/install), clone the repository and run ```cargo build --release``` within the repository folder. Run 
```
./target/release/password_generator <length>
```
to generate a password that uses a-z, A-Z and 0-9. Additional characters can be given with the config flag: ```-c <characters>``` or ```--config <characters>```. A description  of the tool can also be accessed in the command line through the usage of the help flag: ```-h``` or ```--help```.


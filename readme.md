# DelDup - Delete Duplicates

DelDup is Rust program for deleting duplicates.

## Installation
To install you need to have installed `cargo` version 1.73.0 or later.
You can install by using 
```bash
$: cargo install deldup
```
or by building it by yourself
```bash
$: git clone https:/www.github.com/Llamexis/DelDup/
$: cd DelDup
$: cargo build --release
```

## Usage

```bash
Usage: target/debug/deldup [options] target ...
        options:
                -r,             recursive
                -o,             output program will output duplicates in output.json
                -v,             verbose
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)

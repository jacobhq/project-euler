# next

Rust binary to quickly make the next crate for me, and open the problem webpage.

## Usage

Install (run in this directory)

```
cargo install --path .
```

Set up directories like so

```
project/
├─ rs/
│  ├─ pXXXX/
```

And run `next` in the `project` directory. It will generate projects in ascending order in the `rs` directory, and give you the cd command and the Project Euler URL.
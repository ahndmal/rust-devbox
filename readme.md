Trying Rust.

https://doc.rust-lang.org/cargo/commands/cargo-build.html

cli:
```commandline
sudo apt install libssl-dev

cargo build
cargo build --release
cargo run
```

```cli
rustc -O main.rs
```

Set nightly as default:
```
rustup default nightly
rustup default stable
rustup update
```

## Pointers

* Raw Pointer ()
* Box<T> - store value in central storage in a heap
* Rc<T> - shared access to values
* Arc<T> - shared access to values, threadsafe
* Cell<T> - interior mutability
* RefCell<T> - interior mutability + can be nested inside Rc and Arc which only accept immutable refs_
* Cow<T> - avoids writes when only read access is used
* Vec<T> 
* RawVec<T> - works with memory allocator to find space
* Unique<T> - base for types such as Strings, requiring exclusive posession of values
* Shared<T> - shared ownership, can align memmory to T's width even when empty



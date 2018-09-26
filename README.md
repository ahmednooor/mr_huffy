### mr_huffy
A Huffman Encoder written in Rust (An Experimental Project)

### Usage
1) Copy `mr_huffy` dir from `src/` and paste it in your `src/`.
2) Then in your main,
```rust
mod mr_huffy;
fn main() {
    let is_encoded = 
        mr_huffy::encode_file("input/file/path.xyz", "encoded/file/path.xyz");
        // ^ returns a tuple (u8, String)
        // if is_encoded.0 == 0 means success
        // else is_encoded.0 is err code and is_encoded.1 is err msg
    let is_decoded = 
        mr_huffy::decode_file("encoded/file/path.xyz", "decoded/file/path.xyz");
        // ^ returns a tuple (u8, String)
        // if is_decoded.0 == 0 means success
        // else is_decoded.0 is err code and is_decoded.1 is err msg
}
```
> Start from `mod.rs` in `src/mr_huffy/` for source entry point.

> P.S. It is not so fast and works well with only text files.

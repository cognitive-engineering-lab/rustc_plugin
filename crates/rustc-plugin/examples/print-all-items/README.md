# Example: print-all-items

This is an example Rustc plugin that prints all the item names in a given crate. Run the example like this:

```bash
# Install the print-all-items binaries
cd examples/print-all-items
cargo install --path . 

# Run the binaries on an example crate
cd test-crate
cargo print-all-items
```

You should see the output:

```text
There is an item "" of type "`use` import"
There is an item "std" of type "extern crate"
There is an item "add" of type "function"
```
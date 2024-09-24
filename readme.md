# Human Number

This library is just made to format numbers in a pretty human readable way.

## Installation

```bash
cargo add human-number
```

## Usage

```rust
// Using SI scales
let formatter = Formatter::si().with_unit("g");
let result = format!("{}", formatter.format(40_280.0));
assert_eq!(result, "40.28 kg");
let result = format!("{}", formatter.format(0.04823));
assert_eq!(result, "48.23 mg");

// Using binary scales
let formatter = Formatter::binary().with_unit("B");
let result = format!("{}", formatter.format(4096.0));
assert_eq!(result, "4.00 kiB");
```

> Command-line table typewriter.

Example:

```rs
let cell = Cell::new()
    .set_width(20)
    .set_text("Text")
    .set_text_pad('+');
let row = super::Row::new()
    .add_cell(cell)
    .build_rows();
```

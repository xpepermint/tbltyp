> Command-line table typewriter.

Example:

```rs
let column = Column::new()
    .set_width(20)
    .set_text("Text")
    .set_text_pad('+');
let row = Row::new()
    .add_column(column)
    .build_rows();
```

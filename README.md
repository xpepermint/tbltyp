> Table string typewriter.

This library is for creating formatted string tables. It honors ANSI and
Unicode characters thus you can safely use colors and other ANSI styles in
columns. This typewriter is primarily meant for command-line purposes but you
can use it for general string manipulation as well.

## Example

```rs
let column = Column::new()
    .set_width(20)
    .set_text("Hello world!")
    .set_text_pad('+');
let row = Row::new()
    .add_column(column)
    .to_string();
```

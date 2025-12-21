# fuzzy_dir

A fuzzy matching library specifically made for matching folder names.

## Getting started

```rust
let score: i32 = fuzzy_dir::score_dir("this_is_my_folder_name", "myfoldrnam");
```

This library also exports a split function which tries to split
a folder name into its separate words.

This library is used in other projects like:

- [lacy](https://github.com/timothebot/lacy)
- [city](https://github.com/timothebot/city)

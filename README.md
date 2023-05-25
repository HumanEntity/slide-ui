# Slide-ui
Slide-ui is simple presentation tool for markdown in terminal. In short I call it `sui` and It stands for `SlideUI`.

## Installation
For now you have to build it from source. Here are the steps required to do this:

- Clone the repo using
- Cd into it
- And build it

Here is a small snippet to do this in bash

```bash
git clone https://github.com/HumanEntity/slide-ui
cd slide-ui
cargo build --release
```

And optionally install it to your `$PATH`
```bash
cargo install --path .
```

## Configuration
Slide-ui needs `config.toml` file in `$HOME/sui/config.toml` to look good.
The default config I use is here:
```toml
[theme]
h1 = {fg = "red"}
h2 = {fg = "green"}
h3 = {fg = "yellow"}
h4 = {fg = "blue"}
h5 = {fg = "magenta"}
h6 = {fg = "cyan"}
text = {fg = "white"}
```

You can also set `bg` element for background color of the item.
For example:

```toml
[theme]
h1 = {fg = "black", bg = "white"}
```

It would be level 1 heading in black with white background.

All available colors are:

-	red
-	darkred
-	yellow
-	darkyellow
-	blue
-	darkblue
-	green
-	darkgreen
-	grey
-	darkgrey
-	cyan
-	darkcyan
-	magenta
-	darkmagenta
-	white
-	black

For any other string it would be white.

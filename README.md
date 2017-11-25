# Pure-rs

A [Pure](https://github.com/sindresorhus/pure) inspired prompt in [Rust](https://www.rust-lang.org/).


A minimal and fast prompt

## Installation

```zsh
function zle-line-init {
  PROMPT=`/path/to/pure/binary prompt -r "$?"`
  zle reset-prompt
}
zle -N zle-line-init

autoload -Uz add-zsh-hook

function _prompt_pure_rs_precmd() {
  /path/to/pure/binary precmd
}
add-zsh-hook precmd _prompt_pure_rs_precmd
```

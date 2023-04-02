# CPG
#### Consistent Password Generator.

> this project is under development, and hasn't been reviewed by a cryptographer [other limitations](#limitations)

A CLI that consistently generates passwords given the same input.

## concept

CPG is not a password manager, but can be used
as an **additional security layer** on top of it.

The actual password used to sign up, and sign in, to services
**won't be stored anywhere** as it will be recreated every time instead.

```
 input    >       salt    =>    password
   ^                ^              ^
   |                |              |
 store it        genrated      use it to 
 online or       and stored    sign up and sign in
 remember it     locally       to services
```

## installation

```
cargo install cpg
```
see: 
[installing binaries with cargo install](https://doc.rust-lang.org/book/ch14-04-installing-binaries.html), [install rust and cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## basic usage

`-p` list of passwords to salt

`-s` path to the salt to use *(optional)*

`--help` usage help

`--version` cpg version

> this is an example, you should use stronger inputs

```
[9elt@arch ~]$ cpg -p mybirthday iloveyou "mickey mouse"

mybirthday => 0#B[?7WZbufDrh#z{nJXPD8G6pP]BGQk
iloveyou => g3s5PKVbm1NK8UZFlyjxgsA%g5urG05#
mickey mouse => 1,it6rT^%=#geUO853q|€%B#@T4B"fjy
```

### limitations

* there isn't a salt encryption method yet.

* not tested on windows.

* doesn't clean bash_history

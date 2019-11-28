# 🧢 pokedex

[![Latest version](https://img.shields.io/crates/v/pokedex.svg)](https://crates.io/crates/pokedex)

This is primarily a practice project for learning about Rust and Cargo.
You can [follow me on Twitter](https://twitter.com/AriVanider) for stuff about community and Rust and game dev.

If you _really_ want to use this tool, here's how:

1. `cargo install pokedex -f`
2. On the command line, run `pokedex <name_of_pokemon>`
3. Profit

Current response format:

```
ID: 149
Name: dragonite
Height: 2.2m
Weight: 210kg
Types: dragon, flying
```

#### Known Issues

- If you spell the Pokemon's name wrong, you'll get a bad error.
- Gen 8 Pokemon (Sword and Shield) are not yet included as they aren't supported by veekun/pokedex and the PokeAPI yet. [Tracking issue here](https://github.com/veekun/pokedex/issues/284).

## License / Contributing

Project is MIT-licensed and can be used freely.

Not actively seeking PRs or Issues as I'm still learning the language.

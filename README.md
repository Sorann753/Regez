# Regez

[![Rust unit test](https://github.com/Sorann753/Regez/actions/workflows/rust.yml/badge.svg)](https://github.com/Sorann753/Regez/actions/workflows/rust.yml)

## Regex made easy
This is a tool to create regex using english like syntax. the name is pronounced as a combination of reg as in "regex" and ez as in "gg ez"
For now **this project is nowhere near production ready** but if the idea inspire you, feel free to use my code as it is licensed under the MIT license

## Features
- [X] build a regex representation (Regez) using very clear functions, you should be able to make regex like you'd describe them in english
- [X] get the actual regex by turning your Regez into string, making the library compatible with anything
- [ ] whatever you do, this crate should optimize away the useless tokens to give you the optimal regex
- [ ] high level actions to do complex things easily. For example you can use `negate` to create a Regez which will match anything the original didn't
- [ ] use from_string to turn your already existing regex into a Regez and take advantage of the optimizations and high level actions

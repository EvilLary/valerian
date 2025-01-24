# Valerian
simple program to fetch cars from [TheCatAPI](https://thecatapi.com/)

It's a [ccat copycat](https://github.com/plastic-bottleneck/ccat) but way slower and more bloated :)

![hola](assets/hola.jpg)

## Installation
```bash
git clone https://codeberg.org/EvilLary/Valerian.git
cd Valerian
cargo build --release
cp ./target/release/valerian $HOME/.local/bin/
```
## Usage

```bash
valerian <number-of-cars> <output-directory>
```

```bash
valerian 3 $HOME/Downloads/
```

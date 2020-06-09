# Play Sound

Simple CLI tool for playing audio files over selected audio device


## Instalation

Install from this repository using:

``` console
cargo install --git https://github.com/dmweis/play_sound
```

If you are having issues building on linux because of alsa-sys you may need to install `libasound2-dev`

## Usage

To play with default audio device use:

``` console
play_sound -f ./some_file.extension
```

To play over a specificed audio device:

``` console
play_sound -f ./some_file.extension -d "device name"
```

To list audio devices use:

``` console
play_sound -l
```

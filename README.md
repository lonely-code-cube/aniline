# aniline

A cli tool to search, play and download anime.

> Work in progress

## Installation

If you have rust installed on your computer:
```sh
cargo install aniline
```

## Usage

### Search an anime from MAL

```sh
aniline search "anime name"
```

### Play an anime

```sh
aniline play "anime name"
```

Options:

```sh
--vlc, -v - Play video in vlc [default mpv]
--quality, -q - Specify playback quality
```

Example with options:
```sh
aniline play "anime name" -q 720 --vlc
```

> Note: MPV/VLC must be present in path
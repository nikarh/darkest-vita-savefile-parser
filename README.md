# darkest-vita-savefile

A tool to convert [Darkest Dungeon](https://store.steampowered.com/app/262060/) save files from Sone PlayStation Vita format to the one used on other platforms.

## Why

Imagine you want to [edit your save files](https://github.com/robojumper/DarkestDungeonSaveEditor) or continue playing the game on a PC without losing the progress you have made on Vita.

Even if you have a hacked Vita and can decrypt and download save files, unlike on other platforms, where Darkest Dungeon saves are stored as a bunch of files with a `.json` extension (even though they are not Json), on Vita saves are stored in a single `profile_N.z` file, which is a homebrew format that stores all of the "files" in the same file (much like a TLV) using zlib as compression for the contents.

This tool allows easily converting this `.z` file to a folder with Json and vice versa.

## Usage

All of the CLI flags are explain in the help

```
darkest-vita-savefile --help
darkest-vita-savefile encode --help
darkest-vita-savefile decode --help
```


## Example

```
darkest-vita-savefile decode ./profile_0.z ./out
```
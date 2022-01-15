# extractor

Tool to extract log files off of the roboRIO automatically

## Steps

This program does the following steps once ran:

1. Read from [TOML](https://toml.io/en/) config file in `/.config/extractor/`.
2. Run [rsync](https://en.wikipedia.org/wiki/Rsync) to sync any _changed_ log files to a temporary directory.
3. Parse config files stored in temporary directory.

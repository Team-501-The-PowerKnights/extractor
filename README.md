# extractor

[![build](https://github.com/Team-501-The-PowerKnights/extractor/actions/workflows/build.yml/badge.svg)](https://github.com/Team-501-The-PowerKnights/extractor/actions/workflows/build.yml)
[![lint](https://github.com/Team-501-The-PowerKnights/extractor/actions/workflows/lint.yml/badge.svg)](https://github.com/Team-501-The-PowerKnights/extractor/actions/workflows/lint.yml)
[![test](https://github.com/Team-501-The-PowerKnights/extractor/actions/workflows/test.yml/badge.svg)](https://github.com/Team-501-The-PowerKnights/extractor/actions/workflows/test.yml)

Tool to extract and analyze log files off of the roboRIO.

## How It Works

extractor will pull the log files off of your robot's roboRIO using sftp and find which log files are actually from competition matches. With the basic configuration file extractor will do the following steps:

1. Load the configuration file (`conf.toml`).
2. Prepare for using sftp.
3. Sync the files from the source folder (`source_folder` in `conf.toml`) to a local folder.
4. Move log files from competition matches to a folder separate from the other log files.
5. Put the log files from competition matches into a [specific filename based on the competition match it's linked to](#real-log-format).

## Real Log Format

Real log files (log files from competitions) have a specific format to them:

- Current time in YYYY-MM-DD format (e.g. `2022-02-13`).
- Event name (e.g. `NHBB`).
- First character of match type (e.g. `Q`).
- Match number (e.g. `06`).
- Replay number (e.g. `01`).
- First character of alliance (e.g. `R`).
- Location (e.g. `1`).

Example:

```txt
2022-02-13_NHBB_Q-06-01_R1.log
```

## Configuration File

extractor is configured using the [TOML](https://toml.io/en/) configuration format. When running the tool it is loaded from the current working directory from a file called `conf.toml`. Here are the configuration fields:

| **Name**             | **Description**                                                    | **Type** | **Default Value** |
| -------------------- | ------------------------------------------------------------------ | -------- | ----------------- |
| `hostname`           | Hostname of the roboRIO                                            | String   | None, required    |
| `port`               | Port that SFTP is running on for the roboRIO                       | number   | `22`              |
| `destination_folder` | Path where all log files should go before sorting                  | String   | `"./source/"`     |
| `source_folder`      | Path on roboRIO where all the log files are                        | String   | None, required    |
| `username`           | Username to use when connecting to the roboRIO via SFTP            | String   | `"lvuser"`        |
| `password`           | Password to use when connecting to the roboRIO via SFTP            | String   | `""`              |
| `remove`             | If the log files should be removed from the roboRIO after transfer | boolean  | `false`           |
| `real_logs_folder`   | Path to where the real log files should go                         | String   | `"./real/"`       |

### Example

Here is an example of this configuration:

```toml
hostname = "roboRIO-501-FRC"
source_folder = "/media/sda1/501logs/"
remove = true
```

## How to run

Just run the executable.

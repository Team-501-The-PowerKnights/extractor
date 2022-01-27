# extractor

[![build](https://github.com/Team-501-The-PowerKnights/extractor/actions/workflows/build.yml/badge.svg)](https://github.com/Team-501-The-PowerKnights/extractor/actions/workflows/build.yml)
[![lint](https://github.com/Team-501-The-PowerKnights/extractor/actions/workflows/lint.yml/badge.svg)](https://github.com/Team-501-The-PowerKnights/extractor/actions/workflows/lint.yml)
[![test](https://github.com/Team-501-The-PowerKnights/extractor/actions/workflows/test.yml/badge.svg)](https://github.com/Team-501-The-PowerKnights/extractor/actions/workflows/test.yml)

Tool to extract and analyze log files off the roboRIO.

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

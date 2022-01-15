package config

import (
	"os"
	"path/filepath"

	"github.com/BurntSushi/toml"
	"github.com/gleich/lumber/v2"
)

func Read() (Config, error) {
	configDir, err := os.UserConfigDir()
	if err != nil {
		return Config{}, err
	}

	b, err := os.ReadFile(filepath.Join(configDir, "extractor", "conf.toml"))
	if err != nil {
		return Config{}, err
	}
	var data Config
	_, err = toml.Decode(string(b), &data)
	if err != nil {
		return Config{}, err
	}

	lumber.Success("Loaded configuration file")
	return data, nil
}

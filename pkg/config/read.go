package config

import (
	"os"
	"path/filepath"

	"github.com/BurntSushi/toml"
	"github.com/gleich/lumber/v2"
)

func Read() (Config, error) {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return Config{}, err
	}

	b, err := os.ReadFile(filepath.Join(homeDir, ".config", "extractor", "conf.toml"))
	if err != nil {
		return Config{}, err
	}
	var data Config
	_, err = toml.Decode(string(b), &data)
	if err != nil {
		return Config{}, err
	}

	if data.Port == nil {
		defaultPort := 22
		data.Port = &defaultPort
	}
	if data.Destination == nil {
		defaultDestination := filepath.Join(homeDir, "Desktop", "extractor_logs")
		data.Destination = &defaultDestination
	}

	lumber.Success("Loaded configuration file")
	return data, nil
}

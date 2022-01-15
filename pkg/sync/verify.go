package sync

import (
	"errors"
	"os"
	"os/exec"

	"github.com/Team-501-The-PowerKnights/extractor/pkg/config"
)

func VerifyEnv(conf config.Config) error {
	// Create target dir if it doesn't exist
	if _, err := os.Stat(*conf.Destination); errors.Is(err, os.ErrNotExist) {
		err = os.MkdirAll(*conf.Destination, 0755)
		if err != nil {
			return err
		}
	}

	// Check to make sure the user has sftp in their PATH
	_, err := exec.LookPath("sftp")
	if err != nil {
		return err
	}

	return nil
}

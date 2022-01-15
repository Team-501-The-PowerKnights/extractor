package sync

import (
	"fmt"
	"os"
	"os/exec"
	"strings"

	"github.com/Team-501-The-PowerKnights/extractor/pkg/config"
	"github.com/gleich/lumber/v2"
)

func RunSFTP(conf config.Config) error {
	full_loc := fmt.Sprintf("%v@%v.local:%v", *conf.Username, *conf.Hostname, *conf.Source)
	cmd := exec.Command(
		"sftp",
		"-r",
		full_loc,
		*conf.Destination,
	)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	cmd.Stdin = os.Stdin
	err := cmd.Run()
	if err != nil {
		return err
	}
	lumber.Success("Downloaded logs from RIO")

	cmd = exec.Command("sftp", full_loc)
	cmd.Stdin = strings.NewReader("rm logfile-*")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	err = cmd.Run()
	if err != nil {
		return err
	}
	lumber.Success("Removed log located on RIO")

	return nil
}

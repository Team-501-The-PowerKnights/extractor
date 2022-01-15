package cmd

import (
	"github.com/Team-501-The-PowerKnights/extractor/pkg/config"
	"github.com/Team-501-The-PowerKnights/extractor/pkg/sync"
	"github.com/gleich/lumber/v2"
	"github.com/spf13/cobra"
)

var RootCmd = &cobra.Command{
	Use:   "extractor",
	Short: "Tool to extract log files off of the roboRIO automatically",
	Run: func(cmd *cobra.Command, args []string) {
		conf, err := config.Read()
		if err != nil {
			lumber.Fatal(err, "Failed to load config file")
		}

		err = sync.VerifyEnv(conf)
		if err != nil {
			lumber.Fatal(err, "Failed to verify environment")
		}

		err = sync.RunSFTP(conf)
		if err != nil {
			lumber.Fatal(err, "Failed to run sftp")
		}
	},
}

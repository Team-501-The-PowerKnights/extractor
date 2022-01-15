package main

import (
	"github.com/Team-501-The-PowerKnights/extractor/pkg/cmd"
	"github.com/gleich/lumber/v2"
)

func main() {
	err := cmd.RootCmd.Execute()
	if err != nil {
		lumber.Fatal(err, "Failed to execute root command")
	}
}

package cmd

import (
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
	"os"
)

// rootCmd represents the base command when called without any subcommands
var rootCmd = &cobra.Command{
	Use:   "brasse",
	Short: "Homebrew, but better",
	Long: `Brasse is a Homebrew clone, written in Go with the goals of:

  - being faster
  - providing a better piping support
  - being better at managing dependencies
  - being a drop-in replacement of Homebrew

Read more at: <https://codeberg.org/kytta/brasse/>`,
	Version: "0.1.0",
}

func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

func init() {
	cobra.OnInitialize(initConfig)
}

// initConfig reads in config file and ENV variables if set.
func initConfig() {
	viper.SetEnvPrefix("HOMEBREW")
	viper.AutomaticEnv()
}

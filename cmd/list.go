package cmd

import (
	"codeberg.org/kytta/brasse/pkg/brasse"
	"fmt"
	"github.com/fatih/color"
	"github.com/spf13/cobra"
	"log"
	"os"
)

var cyan = color.New(color.FgCyan).SprintFunc()
var bold = color.New(color.Bold).SprintFunc()

// listCmd represents the list command
var listCmd = &cobra.Command{
	Use:   "list",
	Short: "List installed packages",
	Long:  `List all installed formulae and casks.`,
	Run: func(cmd *cobra.Command, args []string) {
		formulae := make(chan string)
		casks := make(chan string)

		go listDirsToChannel(formulae, brasse.GetCellarPath())
		go listDirsToChannel(casks, brasse.GetCaskroomPath())

		fmt.Fprintf(os.Stdout, "%s %s\n", cyan("==>"), bold("Formulae"))

		for {
			formula, more := <-formulae
			if !more {
				break
			}

			fmt.Fprintln(os.Stdout, formula)
		}

		fmt.Fprintln(os.Stdout)
		fmt.Fprintf(os.Stdout, "%s %s\n", cyan("==>"), bold("Casks"))

		for {
			cask, more := <-casks
			if !more {
				break
			}
			fmt.Fprintln(os.Stdout, cask)
		}
	},
}

func listDirsToChannel(channel chan string, dirName string) {
	files, err := os.ReadDir(dirName)
	if err != nil {
		log.Fatal(err)
	}

	for _, file := range files {
		if !file.IsDir() {
			continue
		}
		channel <- file.Name()
	}
	close(channel)
}

func init() {
	rootCmd.AddCommand(listCmd)

	// TODO: --formula, --formulae

	// TODO: --cask, --casks

	// TODO: --full-name

	// TODO: --versions

	// TODO: --multiple

	// TODO: --pinned

	// TODO: -1

	// TODO: -l

	// TODO: -r

	// TODO: -t
}

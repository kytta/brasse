package brasse

import (
	"os"
	"os/exec"
	"strings"
)

func GetCellarPath() string {
	cellarPath, exists := os.LookupEnv("HOMEBREW_CELLAR")
	if !exists {
		// TODO: do not rely on Homebrew
		cellarPathBytes, err := exec.Command("brew", "--cellar").Output()
		if err != nil {
			// TODO: change default value depending on the OS
			cellarPath = "/opt/homebrew/Cellar"
		} else {
			cellarPath = string(cellarPathBytes)
		}
	}

	return strings.TrimSpace(cellarPath)
}

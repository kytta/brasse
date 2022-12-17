package brasse

import (
	"os/exec"
	"strings"
)

func GetCaskroomPath() string {
	var caskroomPath string
	// TODO: do not rely on Homebrew
	caskroomPathBytes, err := exec.Command("brew", "--caskroom").Output()
	if err != nil {
		// TODO: change default value depending on the OS
		caskroomPath = "/opt/homebrew/Caskroom"
	} else {
		caskroomPath = string(caskroomPathBytes)
	}

	return strings.TrimSpace(caskroomPath)
}

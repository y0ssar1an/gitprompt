package main

import (
	"bytes"
	"context"
	"fmt"
	"io/ioutil"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"time"
)

func main() {
	wd, err := os.Getwd()
	if err != nil {
		return
	}

	branch := gitBranch(wd)
	if branch == "" {
		return
	}

	colorized := fmt.Sprintf(" %%F{blue}(%%F{red}%s%%F{blue})%%f", branch)
	dirty := isDirty()
	if err != nil {
		fmt.Println(colorized)
		return
	}

	if dirty {
		colorized += "💩"
	}

	fmt.Println(colorized)
}

func gitBranch(path string) string {
	if strings.Contains(path, string(os.PathSeparator)+".git") {
		return ".git"
	}

	d := findGitDir(path)
	if d == "" {
		return ""
	}

	b, err := ioutil.ReadFile(filepath.Join(d, ".git/HEAD"))
	if err != nil {
		return ""
	}

	i := bytes.Index(b, []byte("refs/heads/")) + len("refs/heads/")
	if i == -1 {
		return ""
	}

	return strings.TrimSpace(string(b[i:]))
}

// findGitDir walks from the current directory to the root directory, returning
// the absolute path to the first .git directory it finds. If no .git directory
// is found, it returns an error.
func findGitDir(path string) string {
	for path != "" {
		_, err := os.Stat(filepath.Join(path, ".git"))
		if err == nil {
			return path
		}

		path = strings.TrimRight(path, string(os.PathSeparator))
		path, _ = filepath.Split(path)
	}

	return ""
}

func isDirty() bool {
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()
	cmd := exec.CommandContext(ctx, "git", "ls-files", "--deleted", "--modified", "--unmerged", "--killed", "--other", "--exclude-standard")
	var buf bytes.Buffer
	cmd.Stdout = &buf

	if err := cmd.Run(); err != nil {
		return false
	}

	stdout := strings.TrimSpace(buf.String())

	// If there's any output, git ls-files has printed filenames. This
	// means there are dirty or untracked files.
	return len(stdout) > 0
}

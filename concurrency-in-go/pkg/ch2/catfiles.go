package ch2

import (
	"fmt"
	"os"
	"strings"
	"time"
)

// One, Write a program  that acceps a list of text file-names as agruments
// For each filename the pogram should spawn a new goroutine that will output the contents of that file to the console
func catfiles(files []string) error {
	for _, file := range files {
		go func() {
			b, err := os.ReadFile(file)
			if err != nil {
				panic(err)
			}
			fmt.Println(string(b))
		}()
	}
	time.Sleep(1 * time.Second)
	return nil
}

func grepfiles(search string, files []string) {
	for _, file := range files {
		go func() {
			b, err := os.ReadFile(file)
			if err != nil {
				panic(err)
			}
			if strings.Contains(string(b), search) {
				fmt.Printf("Found match '%s' in: %s\n", search, file)
			}
		}()
	}
	time.Sleep(2 * time.Second)
}

func grepDir(search, dirName string) error {
	entries, err := os.ReadDir(dirName)
	if err != nil {
		return err
	}
	for _, entry := range entries {
		name := entry.Name()
		fmt.Println(name)

	}
	return nil
}

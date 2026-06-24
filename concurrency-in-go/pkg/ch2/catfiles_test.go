package ch2

import (
	"testing"
)

func TestCatfiles(t *testing.T) {
	files := []string{"../../testdata/file1.txt", "../../testdata/file2.txt", "../../testdata/file3.txt", "../../testdata/file4.txt"}
	err := catfiles(files)
	if err != nil {
		t.Fatalf("expected nil error: %v", err)
	}
}

func TestGrepfiles(t *testing.T) {
	files := []string{"../../testdata/file1.txt", "../../testdata/file2.txt", "../../testdata/file3.txt", "../../testdata/file4.txt"}
	grepfiles("bear", files)
}

func TestGrepdir(t *testing.T) {
	err := grepDir("bear", "../../testdata")
	if err != nil {
		t.Fatal("error not nil: ", err.Error())
	}
}

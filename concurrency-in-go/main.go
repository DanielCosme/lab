package main

import (
	"fmt"
	"sync"
	"time"
)

func countdown(seconds *int, mutex *sync.Mutex) {
	for *seconds > 0 {
		time.Sleep(1 * time.Second)
		mutex.Lock()
		*seconds -= 1
		mutex.Unlock()
	}
}

func main() {
	m := sync.Mutex{}
	// c := sync.NewCond(&m)
	count := 100
	go countdown(&count, &m)
	for {
		time.Sleep(500 * time.Millisecond)
		if m.TryLock() {
			if count <= 0 {
				break
			}
			fmt.Println(count)
			m.Unlock()
		} else {
			print("locked")
		}
	}
}

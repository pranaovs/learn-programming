package main

import (
	"fmt"
	"sync"
	"sync/atomic"
	"time"
)

var wg sync.WaitGroup

var id int32 = 0

func task() {
	atomic.AddInt32(&id, 1)
	taskid := atomic.LoadInt32(&id)
	fmt.Println("Task", taskid, "is running")
	time.Sleep(2 * time.Second)
	fmt.Printf("Task %d is completed\n", id)
}

func main() {
	for i := 1; i <= 5; i++ {
		wg.Go(task)
	}

	wg.Wait()
}

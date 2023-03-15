package main

import "fmt"

type GoEnum = int

const (
	Foo GoEnum = iota
	Bar
	Baz
)

func main() {
	fmt.Println("test")
}

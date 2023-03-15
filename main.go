package main

import "fmt"

type GoEnum = int

const (
	Foo GoEnum = iota
	Bar
	Baz
)

func returnsError(value int) error {
	return fmt.Errorf("this is an error with value %v", value)
}

type SomeStruct struct {
}

func (s *SomeStruct) thisIsOnSomeStruct(value int) error {
	return fmt.Errorf("this is an error with value %v", value)

}

func main() {
	fmt.Println("test")

	x := returnsError(2)
	fmt.Println(x)

	s := SomeStruct{}
	sErr := s.thisIsOnSomeStruct(22)
	fmt.Println(sErr)
}

package main

import (
	"fmt"
	"monkey/repl"
	"os"
	"os/user"
)

func main() {
	user, err := user.Current()
	if err != nil {
		panic(err)
	}

	fmt.Printf("Hello %s! This is the Monkey Lang!\n", user.Username)
	fmt.Printf("Feel free to type commands\n")
	repl.Start(os.Stdin, os.Stdout)

}

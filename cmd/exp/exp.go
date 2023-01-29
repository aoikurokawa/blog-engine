package main

import (
	"os"
	"text/template"
)

type User struct {
	Name string
	Age  int
}

func main() {
	t, err := template.ParseFiles("cmd/exp/hello.gohtml")
	if err != nil {
		panic(err)
	}

	user := User{
		Name: "Ichiro Suzuki",
		Age:  123,
	}

	err = t.Execute(os.Stdout, user)
	if err != nil {
		panic(err)
	}
}

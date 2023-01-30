package views

import (
	"fmt"
	"log"
	"net/http"
	"text/template"
)

type Template struct {
	htmlTpl *template.Template
}

func (t Template) Execute(w http.ResponseWriter, data interface{}) {
	w.Header().Set("Content-Type", "text/html")
	err := t.htmlTpl.Execute(w, data)
	if err != nil {
		log.Printf("executing template: %v", err)
		http.Error(w, "There was an error executing the template", http.StatusInternalServerError)
		return
	}
}

func Parse(filepath string) (Template, error) {
	tpl, err := template.ParseFiles(filepath)
	if err != nil {
		log.Printf("parsing template: %v", err)
		return Template{}, fmt.Errorf("parsing template: %w", err)

	}
	return Template{
			htmlTpl: tpl,
		},
		nil
}

func Must(t Template, err error) Template {
	if err != nil {
		panic(err)
	}
	return t
}

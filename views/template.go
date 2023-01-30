package views

import (
	"fmt"
	"io/fs"
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

func ParseFS(fs fs.FS, filepath string) (Template, error) {
	tpl, err := template.ParseFS(fs, filepath)
	if err != nil {
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

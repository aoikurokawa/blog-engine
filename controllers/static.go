package controllers

import (
	"net/http"
	"os"
	"path/filepath"

	"github.com/Aoi1011/blog/views"
	"github.com/go-chi/chi/v5"
	"github.com/gomarkdown/markdown"
)

type Static struct {
	Template views.Template
}

func (static Static) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	static.Template.Execute(w, nil)
}

func StaticHandler(tpl views.Template) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		tpl.Execute(w, nil)
	}
}

func readMarkdown(post string) []byte {
	content, err := os.ReadFile(filepath.Join("posts", post+".md"))
	if err != nil {
		return []byte("Unable to open")
	}

	md := []byte(content)
	output := markdown.ToHTML(md, nil, nil)
	return output
}

func Blog(tpl views.Template) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		postParam := chi.URLParam(r, "post")
		output := readMarkdown(postParam)

		content := struct{ Content string }{Content: string(output)}

		tpl.Execute(w, content)
	}
}

package main

import (
	"fmt"
	"log"
	"net/http"
	"os"
	"path/filepath"

	"github.com/Aoi1011/blog/views"
	"github.com/go-chi/chi/v5"
	"github.com/gomarkdown/markdown"
)

type Router struct{}

func (router Router) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	switch r.URL.Path {
	case "":
		homeHandler(w, r)
	case "/contact":
		contactHandler(w, r)
	default:
		http.Error(w, "Page not found", http.StatusNotFound)
	}
}

func faqHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "text/html; charset=utf-8")

	// files, err := ioutil.ReadDir("/posts/")
	// if err != nil {
	// 	fmt.Fprintf(w, "unable to open in posts dir: %v", err)
	// 	return
	// }

	// for _, file := range files {
	// 	if file.Name() == "ethereum-basics.md" {

	// 	}
	// }
	content, err := os.ReadFile("/home/aoi/dev/minor/blog/posts/ethereum-basics.md")
	if err != nil {
		fmt.Fprintf(w, "unable to open: %v", err)
		return
	}

	// defer file.Close()

	// scanner := bufio.NewScanner(file)
	// contents := ""

	// for scanner.Scan() {
	// 	content := scanner.Text()
	// 	contents = contents + content

	// }

	md := []byte(content)
	output := markdown.ToHTML(md, nil, nil)
	fmt.Fprintf(w, string(output))
}

func contactHandler(w http.ResponseWriter, r *http.Request) {
	tplPath := filepath.Join("templates", "contact.gohtml")
	executeTemplate(w, tplPath)
}

func homeHandler(w http.ResponseWriter, r *http.Request) {
	tplPath := filepath.Join("templates", "home.gohtml")
	executeTemplate(w, tplPath)
}

func executeTemplate(w http.ResponseWriter, filepath string) {
	tpl, err := views.Parse(filepath)
	if err != nil {
		log.Printf("parsing template: %v", err)
		http.Error(w, "There was an error parsing the template", http.StatusInternalServerError)
		return
	}
	tpl.Execute(w, nil)
}

func main() {
	r := chi.NewRouter()
	r.Get("/", homeHandler)
	r.Get("/contact", contactHandler)
	r.Get("/faq", faqHandler)
	r.NotFound(func(w http.ResponseWriter, r *http.Request) {
		http.Error(w, "Page not found", http.StatusNotFound)
	})
	fmt.Println("Starting the server on :3000...")
	http.ListenAndServe(":3000", r)
}

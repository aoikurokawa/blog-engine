package main

import (
	"fmt"
	"net/http"
	"os"

	"github.com/Aoi1011/blog/controllers"
	"github.com/Aoi1011/blog/templates"
	"github.com/Aoi1011/blog/views"

	"github.com/go-chi/chi/v5"
	"github.com/gomarkdown/markdown"
)

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

func main() {
	r := chi.NewRouter()

	r.Get("/", controllers.StaticHandler(views.Must(views.ParseFS(templates.FS, "home.gohtml", "tailwind.gohtml"))))

	r.Get("/contact", controllers.StaticHandler(views.Must(views.ParseFS(templates.FS, "contact.gohtml"))))

	r.Get("/faq", controllers.FAQ(views.Must(views.ParseFS(templates.FS, "faq.gohtml"))))

	r.NotFound(func(w http.ResponseWriter, r *http.Request) {
		http.Error(w, "Page not found", http.StatusNotFound)
	})
	fmt.Println("Starting the server on :3000...")
	http.ListenAndServe(":3000", r)
}

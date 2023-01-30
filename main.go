package main

import (
	"fmt"
	"net/http"

	"github.com/Aoi1011/blog/controllers"
	"github.com/Aoi1011/blog/templates"
	"github.com/Aoi1011/blog/views"

	"github.com/go-chi/chi/v5"
)

func main() {
	r := chi.NewRouter()

	r.Get("/", controllers.StaticHandler(views.Must(
		views.ParseFS(templates.FS, "home.gohtml", "tailwind.gohtml"))))

	r.Get("/{post}", controllers.Blog(views.Must(
		views.ParseFS(templates.FS, "blog.gohtml", "tailwind.gohtml"))))

	r.Get("/about", controllers.StaticHandler(views.Must(
		views.ParseFS(templates.FS, "about.gohtml", "tailwind.gohtml"))))

	r.NotFound(func(w http.ResponseWriter, r *http.Request) {
		http.Error(w, "Page not found", http.StatusNotFound)
	})
	fmt.Println("Starting the server on :3000...")
	http.ListenAndServe(":3000", r)
}

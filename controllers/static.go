package controllers

import (
	"html/template"
	"net/http"
	"os"

	"github.com/Aoi1011/blog/views"
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

func FAQ(tpl views.Template) http.HandlerFunc {
	questions := []struct {
		Question string
		Answer   template.HTML
	}{
		{
			Question: "Is there a free version?",
			Answer:   "Yes! We offer a free trial for 30 days on any paid plans.",
		},
		{
			Question: "What are your support hours?",
			Answer:   "We have support staff answering emails 24/7, though response times may be a bit slower on weekends.",
		},
		{
			Question: "How do I contact support?",
			Answer:   `Email us - <a href="mailto:support@lenslocked.com">support@lenslocked.com</a>`,
		},
	}
	return func(w http.ResponseWriter, r *http.Request) {
		tpl.Execute(w, questions)
	}
}

func readMarkdown() []byte {
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
		return []byte("Unable to open")
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
	return output
}

func Blog(tpl views.Template) http.HandlerFunc {
	output := readMarkdown()

	content := struct{ Content string }{Content: string(output)}

	return func(w http.ResponseWriter, r *http.Request) {
		tpl.Execute(w, content)
	}
}

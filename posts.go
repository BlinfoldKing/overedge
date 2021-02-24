package main

import (
	"bytes"
	"errors"
	"fmt"
	"io/ioutil"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/gernest/front"
	"github.com/yuin/goldmark"
	"github.com/yuin/goldmark/extension"
)

type Metadata struct {
	Slug       string    `json:"slug"`
	Title      string    `json:"title"`
	Categories []string  `json:"categories"`
	Author     string    `json:"author"`
	Date       time.Time `json:"date"`
}

// Post post type
type Post struct {
	Metadata
	Body string `json:"body"`
}

// PostService service for post operation
type PostService struct {
	posts map[string]Post
}

// Init PostService
func (service *PostService) Init() error {
	service.posts = make(map[string]Post)

	files := make([]string, 0)
	err := filepath.Walk("./posts", func(path string, info os.FileInfo, err error) error {
		if path != "./posts" {
			files = append(files, path)
		}

		return nil
	})

	m := front.NewMatter()
	m.Handle("+++", front.YAMLHandler)
	for _, file := range files {
		content, err := ioutil.ReadFile("./" + file)
		if err != nil {
			return err
		}
		f, body, err := m.Parse(bytes.NewReader(content))
		if err != nil {
			return err
		}

		var buf bytes.Buffer
		markdown := goldmark.New(
			goldmark.WithExtensions(
				extension.DefinitionList,
				extension.Footnote,
				extension.GFM,
				extension.Typographer,
			),
		)
		err = markdown.Convert([]byte(body), &buf)
		if err != nil {
			return err
		}

		date, _ := time.Parse(time.RFC3339, f["date"].(string))

		file = file[6:]
		file = file[:len(file)-3]

		post := Post{
			Metadata: Metadata{
				Slug:       file,
				Title:      f["title"].(string),
				Author:     f["author"].(string),
				Date:       date,
				Categories: strings.Split(f["categories"].(string), ","),
			},
			Body: string(buf.Bytes()),
		}

		fmt.Println(post.Body)
		service.posts[file] = post
	}

	return err
}

// GetBySlug :nodoc
func (service PostService) GetBySlug(slug string) (Post, error) {
	post, ok := service.posts[slug]
	if ok {
		return post, nil
	}

	return post, errors.New("not found")
}

// GetAll :nodoc
func (service PostService) GetAll(query string) ([]Metadata, error) {
	res := make([]Metadata, 0)

	for _, value := range service.posts {
		if strings.Contains(value.Author, query) || strings.Contains(value.Title, query) {
			res = append(res, value.Metadata)
		}
	}

	return res, nil
}

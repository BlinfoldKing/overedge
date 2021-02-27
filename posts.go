package main

import (
	"bytes"
	"errors"
	"io/ioutil"
	"os"
	"path/filepath"
	"sort"
	"strings"
	"time"

	"github.com/gernest/front"
	"github.com/yuin/goldmark"
	"github.com/yuin/goldmark/extension"
	"github.com/yuin/goldmark/renderer/html"
)

type Metadata struct {
	Slug       string    `json:"slug"`
	Title      string    `json:"title"`
	Subtitle   string    `json:"subtitle"`
	Thumbnail  string    `json:"thumbnail"`
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
			goldmark.WithRendererOptions(html.WithUnsafe()),
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
				Subtitle:   f["subtitle"].(string),
				Thumbnail:  f["thumbnail"].(string),
				Author:     f["author"].(string),
				Date:       date,
				Categories: strings.Split(f["categories"].(string), ","),
			},
			Body: string(buf.Bytes()),
		}

		if f["status"] != "draft" {
			service.posts[file] = post
		}
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

	query = strings.ToLower(query)

	for _, value := range service.posts {
		if strings.Contains(strings.ToLower(value.Author), query) || strings.Contains(strings.ToLower(value.Title), query) {
			res = append(res, value.Metadata)
		}
	}

	sort.SliceStable(res, func(i, j int) bool {
		return res[i].Date.After(res[j].Date) && res[i].Slug > res[j].Slug
	})

	return res, nil
}

package main

import (
	"bytes"
	"encoding/json"
	"errors"
	"io/ioutil"
	"strings"
	"time"

	"github.com/gernest/front"
	"github.com/yuin/goldmark"
	"github.com/yuin/goldmark/extension"
	"github.com/yuin/goldmark/renderer/html"
)

type Recommendation struct {
	Slug  string `json:"slug"`
	Title string `json:"title"`
}

type Metadata struct {
	Slug       string          `json:"slug"`
	Title      string          `json:"title"`
	Subtitle   string          `json:"subtitle"`
	Thumbnail  string          `json:"thumbnail"`
	Categories []string        `json:"categories"`
	Author     string          `json:"author"`
	Next       *Recommendation `json:"next"`
	Prev       *Recommendation `json:"prev"`
	Date       time.Time       `json:"date"`
}

var published []string

type publishedPost struct {
	Posts []string `json:"posts"`
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

	postsFile, _ := ioutil.ReadFile("posts/posts.json")
	posts := publishedPost{}
	json.Unmarshal(postsFile, &posts)

	m := front.NewMatter()
	published = posts.Posts
	m.Handle("+++", front.YAMLHandler)
	for _, file := range posts.Posts {
		content, err := ioutil.ReadFile("posts/" + file + ".md")
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

		// file = file[6:]
		// file = file[:len(file)-3]

		post := Post{
			Metadata: Metadata{
				Slug:       file,
				Title:      f["title"].(string),
				Subtitle:   f["subtitle"].(string),
				Thumbnail:  f["thumbnail"].(string),
				Author:     f["author"].(string),
				Date:       date,
				Categories: strings.Split(f["categories"].(string), ","),
				Prev:       nil,
				Next:       nil,
			},
			Body: string(buf.Bytes()),
		}

		if f["status"] != "draft" {
			service.posts[file] = post
		}
	}

	var prev string = ""
	for slug := range service.posts {
		currPost := service.posts[slug]
		if prev != "" {
			prevPost := service.posts[prev]
			prevPost.Next = &Recommendation{
				Slug:  currPost.Slug,
				Title: currPost.Title,
			}
			service.posts[prev] = prevPost

			currPost.Prev = &Recommendation{
				Slug:  prevPost.Slug,
				Title: prevPost.Title,
			}
			service.posts[slug] = currPost
		}

		prev = slug
	}

	return nil
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

	for _, value := range published {
		post := service.posts[value]
		if strings.Contains(strings.ToLower(post.Author), query) || strings.Contains(strings.ToLower(post.Title), query) {
			res = append(res, post.Metadata)
		}
	}

	return res, nil
}

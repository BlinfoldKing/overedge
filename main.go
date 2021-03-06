package main

import (
	"os"

	"github.com/joho/godotenv"
	"github.com/labstack/echo"
	"github.com/labstack/echo/middleware"
)

func main() {
	godotenv.Load()

	port := os.Getenv("PORT")

	service := PostService{}
	err := service.Init()
	if err != nil {
		panic(err)
	}

	e := echo.New()
	e.Static("/", "dist/")
	e.Static("/post*", "dist/")
	e.Static("/post/:slug", "dist/")
	e.Static("/static", "static/")

	api := e.Group("/api")
	api.Use(middleware.CORSWithConfig(middleware.CORSConfig{
		AllowOrigins: []string{
			"http://localhost:" + port,
			"https://localhost:" + port,
			"https://blinfoldking.dev",
		},
	}))

	api.GET("/posts", func(ctx echo.Context) error {
		query := ctx.QueryParam("query")
		post, err := service.GetAll(query)
		if err != nil {
			return ctx.JSON(404, echo.Map{
				"ok": false,
				"data": echo.Map{
					"message": "not found",
				},
			})
		}

		return ctx.JSON(200, echo.Map{
			"ok":   true,
			"data": post,
		})
	})
	api.GET("/posts/:slug", func(ctx echo.Context) error {
		slug := ctx.Param("slug")
		post, err := service.GetBySlug(slug)
		if err != nil {
			return ctx.JSON(404, echo.Map{
				"ok": false,
				"data": echo.Map{
					"message": "not found",
				},
			})
		}

		return ctx.JSON(200, echo.Map{
			"ok":   true,
			"data": post,
		})
	})

	e.Static("/*", "dist/")
	e.Logger.Fatal(e.Start(":" + port))
}

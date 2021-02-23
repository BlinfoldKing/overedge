package main

import (
	"os"

	"github.com/labstack/echo"
)

func main() {
	service := PostService{}
	err := service.Init()
	if err != nil {
		panic(err)
	}

	e := echo.New()
	e.Static("/", "./dist")

	api := e.Group("/api")

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

		return ctx.JSON(404, echo.Map{
			"ok":   false,
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

		return ctx.JSON(404, echo.Map{
			"ok":   false,
			"data": post,
		})
	})
	e.Logger.Fatal(e.Start(":" + os.Getenv("PORT")))
}

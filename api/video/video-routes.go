package video

import "github.com/gofiber/fiber/v2"

func VideoRoutes(app fiber.Router) {
	router := app.Group("/api/video")

	// GET
	router.Get("/:videoName", GetVideosHandler)
}
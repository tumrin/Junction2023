package card

import "github.com/gofiber/fiber/v2"

func CardRoutes(app fiber.Router) {
	router := app.Group("/api/card")

	// GET
	router.Get("/", GetCardsHandler)
	router.Get("/:id", GetSingleCardHandler)
}
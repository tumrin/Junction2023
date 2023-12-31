package user

import "github.com/gofiber/fiber/v2"

func UserRoute(app fiber.Router) {
	router := app.Group("/api/user")

	// GET
	router.Get("/create", CreateUserHandler)
	router.Get("/:id", GetUserHandler)
	router.Get("/:id/active-card", GetUserActiveCardHandler)

	//PUT
	router.Put("/:id", UpdateUserHandler)
}
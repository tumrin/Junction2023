package user

import "github.com/gofiber/fiber/v2"

func GetUserHandler(c *fiber.Ctx) error {
	return c.SendString("User")
}
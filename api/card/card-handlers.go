package card

import (
	"github.com/go-playground/validator/v10"
	"github.com/gofiber/fiber/v2"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

var validate = validator.New(validator.WithRequiredStructEnabled()) 

func GetCardHandler(c *fiber.Ctx) error {
	return c.SendString("Test")
}

func GetSingleCardHandler(c *fiber.Ctx) error {
	id := c.Params("id")

	err := validate.Var(id, "required,mongodb")

	if err != nil {
		return c.Status(fiber.StatusBadRequest).SendString("id validaton failed")
	}

	_id, err := primitive.ObjectIDFromHex(id)
	
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).SendString("id conversion failed")
	}

	card, err := GetSingleCard(_id)
	
	if err != nil {
		return c.Status(fiber.StatusNotFound).SendString(err.Error())
	}

	return c.Status(fiber.StatusOK).JSON(card)
}

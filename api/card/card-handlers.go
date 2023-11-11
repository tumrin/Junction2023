package card

import (
	"github.com/go-playground/validator/v10"
	"github.com/gofiber/fiber/v2"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

var validate = validator.New(validator.WithRequiredStructEnabled()) 

func GetCardsHandler(c *fiber.Ctx) error {
	card, err := FetchRandomCard()

	if err != nil {
		return err
	}

	return c.Status(fiber.StatusOK).JSON(card)
}

func GetSingleCardHandler(c *fiber.Ctx) error {
	id := c.Params("id")

	err := validate.Var(id, "required,mongodb")

	if err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(err.Error())
	}

	_id, err := primitive.ObjectIDFromHex(id)

	if err != nil {
		return c.Status(fiber.StatusInternalServerError).SendString("id conversion failed")
	}

	card, err := FetchSingleCard(_id)
	
	if err != nil {
		return err
	}

	return c.Status(fiber.StatusOK).JSON(card)
}

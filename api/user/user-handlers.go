package user

import (
	"github.com/go-playground/validator/v10"
	"github.com/gofiber/fiber/v2"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

var validate = validator.New(validator.WithRequiredStructEnabled()) 

func GetUserHandler(c *fiber.Ctx) error {
	id := c.Params("id")

	err := validate.Var(id, "required,mongodb")

	if err != nil {
		return c.Status(fiber.StatusBadRequest).SendString("id validaton failed")
	}

	_id, err := primitive.ObjectIDFromHex(id)
	
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).SendString("id conversion failed")
	}

	user, err := FetchUserInfo(_id)

	if err != nil {
		return err
	}

	return c.Status(fiber.StatusOK).JSON(user)
}

func UpdateUserHandler(c *fiber.Ctx) error {
	var body UserInfoPutRequest
	id := c.Params("id")

	err := validate.Var(id, "required,mongodb")

	if err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(err.Error())
	}

	_id, err := primitive.ObjectIDFromHex(id)

	if err != nil {
		return c.Status(fiber.StatusInternalServerError).SendString("id conversion failed")
	}

	if err = c.BodyParser(&body); err != nil {
		return fiber.NewError(fiber.ErrBadRequest.Code, err.Error())
	}

	err = validate.Struct(body)

	if err != nil {
		return fiber.NewError(fiber.ErrBadRequest.Code, err.Error())
	}

	err = UpdateUserInfo(_id, body)

	if err != nil {
		return err
	}

	return c.SendStatus(fiber.StatusOK)
}

func GetUserActiveCardHandler(c *fiber.Ctx) error {
	id := c.Params("id")

	err := validate.Var(id, "required,mongodb")

	if err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(err.Error())
	}

	_id, err := primitive.ObjectIDFromHex(id)

	if err != nil {
		return c.Status(fiber.StatusInternalServerError).SendString("id conversion failed")
	}

	card, err := FetchActiveCard(_id)

	if err != nil {
		return err
	}

	return c.Status(fiber.StatusOK).JSON(card)
}

func CreateUserHandler(c *fiber.Ctx) error {
	newUserId, err := CreateNewUser()

	if err != nil {
		return err
	}

	return c.Status(fiber.StatusOK).JSON(newUserId)
}
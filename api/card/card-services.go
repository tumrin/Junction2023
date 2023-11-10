package card

import (
	"context"
	"junction-api/db"

	"github.com/gofiber/fiber/v2"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

func GetSingleCard(_id primitive.ObjectID) (Card, error) {

	filter := bson.D{{Key: "_id", Value: _id}}

	var card Card

	err := db.DbRef.Collection("card").FindOne(context.TODO(), filter).Decode(&card)

	if err != nil {
		fiber.NewError(fiber.ErrNotFound.Code, "missing card")
	}

	return card, err
}
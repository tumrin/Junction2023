package card

import (
	"context"
	"junction-api/db"

	"github.com/gofiber/fiber/v2"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
)

func FetchSingleCard(_id primitive.ObjectID) (db.CardInfo, error) {

	filter := bson.D{{Key: "_id", Value: _id}}

	var card db.CardInfo

	err := db.DbRef.Collection("card").FindOne(context.TODO(), filter).Decode(&card)

	if err != nil {
		return card, fiber.NewError(fiber.ErrNotFound.Code, "missing card")
	}

	return card, nil
}

func FetchRandomCard() (db.CardInfo, error) {
	aggregation := bson.D{{Key: "$sample", Value: bson.D{{Key: "size", Value: 1}}}}

	var card db.CardInfo

	cursor, err := db.DbRef.Collection("card").Aggregate(context.TODO(), mongo.Pipeline{aggregation})

	if err != nil {
		return card, fiber.NewError(fiber.StatusInternalServerError, "could not fetch card")
	}
	defer cursor.Close(context.TODO())

	for cursor.Next(context.TODO()) {
		if err = cursor.Decode(&card); err != nil {
			return card, fiber.NewError(fiber.StatusInternalServerError, "could not decode card")
		}
	}

	if err = cursor.Err(); err != nil {
		return card, fiber.NewError(fiber.StatusInternalServerError, "error occuerd on card parsing")
	}

	return card, nil
}
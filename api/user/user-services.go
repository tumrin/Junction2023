package user

import (
	"context"
	"junction-api/db"

	"github.com/gofiber/fiber/v2"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

func GetUserInfo(_id primitive.ObjectID) (UserInfo, error) {
	filter := bson.D{{Key: "_id", Value: _id}}

	var user UserInfo

	err := db.DbRef.Collection("user").FindOne(context.TODO(), filter).Decode(&user)

	if err != nil {
		return user, fiber.NewError(fiber.StatusInternalServerError, "Error when fetching user data")
	}

	return user, nil
}
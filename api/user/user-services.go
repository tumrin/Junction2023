package user

import (
	"context"
	"junction-api/db"
	"strconv"

	"github.com/gofiber/fiber/v2"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

func FetchUserInfo(_id primitive.ObjectID) (db.UserInfo, error) {
	filter := bson.D{{Key: "_id", Value: _id}}

	var user db.UserInfo

	err := db.DbRef.Collection("user").FindOne(context.TODO(), filter).Decode(&user)

	if err != nil {
		return user, fiber.NewError(fiber.StatusInternalServerError, "Error when fetching user data")
	}

	return user, nil
}

func UpdateUserInfo(_id primitive.ObjectID, data UserInfoPutRequest) error {
	var oldUserData db.UserInfo
	
	oldUserData, err := FetchUserInfo(_id)

	if err != nil {
		return fiber.NewError(fiber.ErrInternalServerError.Code, "could not find user")
	}	

	updatedUser := db.UserInfo {
		Id: _id,
		Name: oldUserData.Name,
		CardInProgress: data.CardInProgress,
		LikedCards: data.LikedCards,
	}

	filter := bson.D{{Key: "_id", Value: _id }}

	if _, err = db.DbRef.Collection("user").ReplaceOne(context.TODO(), filter, updatedUser); err != nil {
		return fiber.NewError(fiber.ErrInternalServerError.Code, "could not update user info")
	}

	return err
}

func FetchActiveCard(_id primitive.ObjectID) (db.CardInfo, error) {
	var card db.CardInfo
	
	userInfo, err := FetchUserInfo(_id)

	if err != nil {
		return card, fiber.NewError(fiber.StatusNotFound, err.Error())
	}

	filter := bson.D{{Key: "_id", Value: userInfo.CardInProgress}}

	err = db.DbRef.Collection("card").FindOne(context.TODO(), filter).Decode(&card)

	if err != nil {
		return card, fiber.NewError(fiber.ErrNotFound.Code, "missing active card")
	}

	return card, nil
}

func CreateNewUser() (NewUserId, error) {
	var newUserId NewUserId

	count, err := db.DbRef.Collection("user").CountDocuments(context.TODO(), bson.D{})

	if err != nil {
		return newUserId, fiber.NewError(fiber.StatusInternalServerError, "error when creating user")
	}

	count++

	newUser := db.UserInfo {
		Name: "User" + strconv.Itoa(int(count)),
	}

	result, err := db.DbRef.Collection("user").InsertOne(context.TODO(), newUser)
	
	if err != nil {
		return newUserId, fiber.NewError(fiber.StatusInternalServerError, "error inserting new user")
	}

	newUserId.Id = result.InsertedID.(primitive.ObjectID)

	return newUserId, nil

}
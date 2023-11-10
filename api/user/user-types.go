package user

import "go.mongodb.org/mongo-driver/bson/primitive"

type UserInfo struct {
	Id primitive.ObjectID `json:"id" bson:"_id" validate:"required"`
	Name string `json:"username" bson:"username" validate:"required"`
	CardInProgress primitive.ObjectID `json:"inProgress" bson:"inProgress,omitempty"`
	LikedCards []primitive.ObjectID `json:"likedCards" bson:"likedCards,omitempty"`
}
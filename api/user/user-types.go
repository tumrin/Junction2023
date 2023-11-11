package user

import "go.mongodb.org/mongo-driver/bson/primitive"

type UserInfoPutRequest struct {
	CardInProgress primitive.ObjectID `json:"inProgress,omitempty" bson:"inProgress,omitempty" validate:"required"`
	LikedCards []primitive.ObjectID `json:"likedCards,omitempty" bson:"likedCards,omitempty" validate:"required,unique"`
}

type NewUserId struct {
	Id primitive.ObjectID `json:"id,omitempty" bson:"_id,omitempty" validate:"required"`
}
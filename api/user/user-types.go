package user

import "go.mongodb.org/mongo-driver/bson/primitive"

type UserInfo struct {
	Id primitive.ObjectID `json:"id,omitempty" bson:"_id,omitempty" validate:"required"`
	Name string `json:"username" bson:"username" validate:"required"`
	CardInProgress primitive.ObjectID `json:"inProgress,omitempty" bson:"inProgress,omitempty"`
	LikedCards []primitive.ObjectID `json:"likedCards,omitempty" bson:"likedCards,omitempty"`
}

type UserInfoPutRequest struct {
	CardInProgress primitive.ObjectID `json:"inProgress" bson:"inProgress" validate:"required"`
	LikedCards []primitive.ObjectID `json:"likedCards" bson:"likedCards" validate:"required,unique"`
}

type NewUserId struct {
	Id primitive.ObjectID `json:"id,omitempty" bson:"id,omitempty" validate:"required"`
}
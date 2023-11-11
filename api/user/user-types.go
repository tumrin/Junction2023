package user

import "go.mongodb.org/mongo-driver/bson/primitive"

type UserInfoPutRequest struct {
	CardInProgress primitive.ObjectID `json:"inProgress" bson:"inProgress" validate:"required"`
	LikedCards []primitive.ObjectID `json:"likedCards" bson:"likedCards" validate:"required,unique"`
}

type NewUserId struct {
	Id primitive.ObjectID `json:"id,omitempty" bson:"_id,omitempty" validate:"required"`
}
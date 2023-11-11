package db

import "go.mongodb.org/mongo-driver/bson/primitive"

type CardInfo struct {
	Id primitive.ObjectID `json:"id,omitempty" bson:"_id,omitempty" validate:"required"`
	VideoLink string `json:"vidLink" bson:"vidLink,omitempty"`
	References []string `json:"reference" bson:"reference,omitempty"`
	Title string `json:"title" bson:"title"`
	Content string `json:"content" bson:"content"`
}

type UserInfo struct {
	Id primitive.ObjectID `json:"id,omitempty" bson:"_id,omitempty" validate:"required"`
	Name string `json:"username" bson:"username" validate:"required"`
	CardInProgress primitive.ObjectID `json:"inProgress,omitempty" bson:"inProgress,omitempty"`
	LikedCards []primitive.ObjectID `json:"likedCards,omitempty" bson:"likedCards,omitempty"`
}
package card

import "go.mongodb.org/mongo-driver/bson/primitive"

type CardInfo struct {
	Id primitive.ObjectID `json:"id" bson:"id" validate:"required"`
	VideoLink string `json:"vidLink" bson:"vidLink,omitempty"`
	References []string `json:"reference" bson:"reference,omitempty"`
	Title string `json:"title" bson:"title"`
	Content string `json:"content" bson:"content"`
}
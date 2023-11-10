package card

import "go.mongodb.org/mongo-driver/bson/primitive"

type Card struct {
	Id primitive.ObjectID `json:"id" bson:"_id" validate:"required"`
	VideoLink string `json:"vidLink" bson:"vidLink,omitempty"`
	References []string `json:"reference:" bson:"reference,omitempty"`
	Title string `json:"title" bson:"title"`
	Content string `json:"content" bson:"content"`
}
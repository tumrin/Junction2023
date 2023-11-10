package db

import (
	"context"
	"junction-api/utils"
	"log"

	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

var DbRef *mongo.Database

func InitDb() {
	log.Println("Starting database")

	bsonOpts := &options.BSONOptions {
		UseJSONStructTags: true,
		OmitZeroStruct: true,
	}
	
	clientOptions := options.Client().ApplyURI(utils.Config.DbUri).SetBSONOptions(bsonOpts)

	client, err := mongo.Connect(context.TODO(), clientOptions)

	if err != nil {
		log.Fatal(err)
	}

	err = client.Ping(context.TODO(), nil)

	if err != nil {
		log.Fatal(err)
	}

	log.Println("Connected to database")

	DbRef = client.Database("junc-api-db")

	// Create collections
	DbRef.CreateCollection(context.TODO(), "card")
	DbRef.CreateCollection(context.TODO(), "user")
}
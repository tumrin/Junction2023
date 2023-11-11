package db

import (
	"context"
	"junction-api/utils"
	"log"

	"go.mongodb.org/mongo-driver/bson"
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
	//TODO: add proper error handling
	DbRef.CreateCollection(context.TODO(), "card")
	DbRef.CreateCollection(context.TODO(), "user")

	err = initCardData()

	if err != nil {
		log.Fatal("error occured when inserting mock cards to db")
	}
}

func initCardData() error {

	count, err := DbRef.Collection("card").CountDocuments(context.TODO(),bson.D{})

	if err != nil {
		log.Fatal("could not get database document count on init")
	}

	if count != 0 {
		return nil
	}

	log.Println("Inserting mock card data...")

	documents := []interface{}{
		CardInfo{
			Title: "Cold/hot treatment",
			Content: "Sauna sessions can be beneficial for individuals with chronic pain as the heat promotes muscle relaxation, easing tension and reducing pain. The induced sweating helps flush out toxins, and the overall warmth enhances blood circulation, fostering a soothing effect that may provide relief from persistent discomfort.",
			References: []string{"reference 1.", "reference 2."},
			VideoLink: "sauna.mp4",
		},
		CardInfo{
			Title: "Walking in forest",
			Content: "Walking in a forest is beneficial for chronic pain sufferers as the tranquil surroundings reduce stress, promoting relaxation and easing pain perception. The gentle exercise of walking also improves circulation and releases endorphins, contributing to an overall sense of well-being and potentially mitigating the impact of chronic pain.",
			References: []string{"reference 3."},
			VideoLink: "forest.mp4",
		},
		CardInfo{
			Title: "Cold and hot showers",
			Content: "Alternating between hot and cold showers can benefit those with chronic pain by improving blood circulation and reducing inflammation. The heat helps relax muscles, while the cold water decreases swelling, creating a contrast that may alleviate pain and enhance overall mobility.",
			References: []string{"reference 4."},
			VideoLink: "temperature.mp4",
		},
		CardInfo{
			Title: "Stretching your back and legs",
			Content: "Regular stretching improves posture, reduces strain on the body, and helps reduce stress and anxiety. All these factors contribute to a decrease in chronic pain. Incorporating stretching into one's daily routine can have a significant impact on overall health and well-being.",
			References: []string{"reference 5."},
			VideoLink: "stretch1.mp4",
		},
		CardInfo{
			Title: "Strength training",
			Content: "Strength training can help individuals manage chronic pain by building strong muscles and bones, which can support and protect vulnerable joints and ligaments, reducing the risk of injury and alleviating pain caused by weakness and instability. Additionally, strength training can improve posture and balance, reducing the strain on painful areas and enhancing overall functional ability.",
			References: []string{"reference 4."},
			VideoLink: "aerobic.mp4",
		},
		CardInfo{
			Title: "Mindfulness and meditation",
			Content: "Mindfulness and meditation can be valuable tools for individuals with chronic pain, helping them to better manage their symptoms and improve their overall quality of life. Mindfulness practices, such as paying attention to the present moment without judgment, can help individuals become more aware of their thoughts, emotions, and bodily sensations, allowing them to respond to pain in a more skillful way.",
			References: []string{"reference 2."},
			VideoLink: "meditation.mp4",
		},
	}

	if _, err = DbRef.Collection("card").InsertMany(context.TODO(), documents); err != nil {
		log.Fatal("error inserting mock data to db" + err.Error())
	}

	log.Println("Mock data completed!")

	return nil
}
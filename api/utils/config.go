package utils

import (
	"log"
	"os"

	"github.com/joho/godotenv"
)

var Config ConfigStuct

type ConfigStuct struct {
	DbUri string
	Port string
}

func InitConfig() {
	log.Println("Initializing config")

	err := godotenv.Load(".env")

	if err != nil {
		log.Fatal("Error loading .env file")
	}

	Config.DbUri = os.Getenv("MONGO_URI")
	Config.Port = os.Getenv("PORT")

	log.Println("Config done")
}
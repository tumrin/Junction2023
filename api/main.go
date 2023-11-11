package main

import (
	"junction-api/card"
	"junction-api/db"
	"junction-api/user"
	"junction-api/utils"
	"junction-api/video"
	"log"

	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/cors"
	"github.com/gofiber/fiber/v2/middleware/helmet"
)

func main() {
	log.Println("Staring server")
	app := fiber.New()
	
	// config
	utils.InitConfig()
	
	// database
	db.InitDb()

	helmetOpts := helmet.Config {
		CrossOriginResourcePolicy: "*",
	}

	// middlewares
	app.Use(helmet.New(helmetOpts))
	app.Use(cors.New())	

	// routes
	card.CardRoutes(app)
	user.UserRoute(app)
	video.VideoRoutes(app)
	
	// simple 404-handler
	app.Use(func (c *fiber.Ctx) error {
		return c.SendStatus(404)
	})

	app.Listen(":"+ utils.Config.Port)
}
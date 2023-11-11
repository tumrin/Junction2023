package video

import "github.com/gofiber/fiber/v2"

func GetVideosHandler(c *fiber.Ctx) error {
	vname := c.Params("videoName", "no-video")

	if vname == "no-video" {
		return c.Status(fiber.ErrBadRequest.Code).SendString("no video name")
	}

	file := "./files/" + vname

	return c.SendFile(file, true)
}
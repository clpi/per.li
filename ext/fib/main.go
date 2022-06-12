package main

import (
	"fmt"

	"github.com/gofiber/fiber/v2"
)

func main() {
	fmt.Println("Welcome to my program")
	app := fiber.New()

	app.Get("/", func(c *fiber.Ctx) error {
		return c.SendString("<h2>Hello world!</h2>")
	})

	app.Get("/about", func(c *fiber.Ctx) error {
		return c.SendString("Hi from \"/about\"!")
	})

	app.Listen(":9876")
}

package main;

import (
    "github.com/gofiber/fiber/v2"
    "github.com/gofiber/fiber/v2/middleware/cors"
    // "net/http"
    "log"
    "fmt"
)

func onlyJson(c *fiber.Ctx) error {
    if c.Is("json") {
        return c.Next()
    } else {
        return c.SendString("Only JSON please");
    }
}

func main() {
    fmt.Println("Welcome to per.li v0.0.1")

    app := fiber.New()
    app.Use(cors.New())

    app.Static("/", "./public")
    app.Get("/docs", func(c *fiber.Ctx) error {
        return c.SendString("<h1>Hello world</h1>")
    })
    app.Get("/user/:username", func(c *fiber.Ctx) error {
        return c.SendString("Param: " + c.Params("username"))
    })
    app.Get("/about", func(c *fiber.Ctx) error {
        return c.SendString("<h2>About</h2>")
    })
    log.Fatal(app.Listen(":3002"))

}

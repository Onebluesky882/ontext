// Package httpapi exposes a local-only debug/status HTTP server backed by
// Fiber. It mirrors a subset of the Wails-bound pipeline controls so the
// pipeline state can be inspected or driven without the desktop UI.
package httpapi

import (
	"context"

	"github.com/gofiber/fiber/v2"

	"ontext-wails/internal/pipeline"
)

// Server is a local debug HTTP API in front of a Pipeline.
type Server struct {
	app      *fiber.App
	pipeline *pipeline.Pipeline
}

// New creates a Server bound to the given pipeline. Routes are registered
// but the server is not yet listening — call Listen to start it.
func New(p *pipeline.Pipeline) *Server {
	app := fiber.New(fiber.Config{DisableStartupMessage: true})

	s := &Server{app: app, pipeline: p}
	s.routes()
	return s
}

func (s *Server) routes() {
	s.app.Get("/health", func(c *fiber.Ctx) error {
		return c.JSON(fiber.Map{"status": "ok"})
	})

	s.app.Post("/pipeline/start", func(c *fiber.Ctx) error {
		result := s.pipeline.Start(c.Context())
		if result.Error != nil {
			return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
				"error": result.Error.Error(),
			})
		}
		return c.JSON(fiber.Map{"text": result.Text})
	})

	s.app.Post("/pipeline/stop", func(c *fiber.Ctx) error {
		if err := s.pipeline.Stop(); err != nil {
			return c.Status(fiber.StatusConflict).JSON(fiber.Map{"error": err.Error()})
		}
		return c.JSON(fiber.Map{"status": "stopping"})
	})
}

// Listen starts the server on addr (e.g. "127.0.0.1:34115"). It blocks until
// the server stops or returns an error.
func (s *Server) Listen(addr string) error {
	return s.app.Listen(addr)
}

// Shutdown gracefully stops the server.
func (s *Server) Shutdown(ctx context.Context) error {
	return s.app.ShutdownWithContext(ctx)
}

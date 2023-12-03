package main

import (
	"app/handler"
	"net/http"

	"github.com/labstack/echo/v4"
)

func dummy(c echo.Context) error {
	return c.JSON(http.StatusOK, map[string]string{"status": "alive"})
}

func main() {
	e := echo.New()

	public := e.Group("/public")
	{
		public.GET("/health/alive", handler.HealthAlive)
		public.GET("/calendar/all", handler.PublicCalendarAll)
		public.GET("/calendar/:id", handler.PublicCalendar)
		public.GET("/calendar/download/:id", handler.PublicCalendarDownload)
		public.GET("/calendar/event/recent", handler.PublicEventRecent)
	}

	admin := e.Group("/admin/calendar")
	{
		admin.GET("/details", handler.AdminCalendarDetails)
		admin.POST("/details", handler.AdminCalendarDetailCreate)
		admin.GET("/details/:id", handler.AdminCalendarDetail)
		admin.PUT("/details/:id", handler.AdminCalendarDetailUpdate)
		admin.PATCH("/details/:id", handler.AdminCalendarDetailUpdate)
		admin.DELETE("/details/:id", handler.AdminCalendarDetailDelete)

		admin.GET("/events", handler.AdminCalendarEvents)
		admin.POST("/events", handler.AdminCalendarEventCreate)
		admin.GET("/events/:id", handler.AdminCalendarEvent)
		admin.PUT("/events/:id", handler.AdminCalendarEventUpdate)
		admin.PATCH("/events/:id", handler.AdminCalendarEventUpdate)
		admin.DELETE("/events/:id", handler.AdminCalendarEventDelete)
	}

	e.Logger.Fatal(e.Start(":3000"))
}

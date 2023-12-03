package handler

import (
	"net/http"

	"github.com/labstack/echo/v4"
)

func HealthAlive(c echo.Context) error {
	response := map[string]string{"status": "alive"}
	return c.JSON(http.StatusOK, response)
}

func PublicEventRecent(c echo.Context) error {
	events := fetchRecentEvents()
	return c.JSON(http.StatusOK, events)
}

func PublicCalendarAll(c echo.Context) error {
	cals := fetchAllCalendars()
	return c.JSON(http.StatusOK, cals)
}

func PublicCalendarDownload(c echo.Context) error {
	id := c.Param("id")
	cal := calendarWithEvents(id)
	c.Response().Header().Set(echo.HeaderContentDisposition, "attachment; filename="+cal.Id+".ics")
	c.Response().Header().Set(echo.HeaderContentType, "text/plain")

	return c.String(http.StatusOK, cal.ToIcal())
}

func PublicCalendar(c echo.Context) error {
	id := c.Param("id")
	cal := fetchCalendar(id)
	return c.JSON(http.StatusOK, cal)
}

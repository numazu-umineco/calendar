package handler

import (
	"app/db"
	"app/model"
	"fmt"
	"net/http"

	"github.com/labstack/echo/v4"
)

func AdminCalendarDetails(c echo.Context) error {
	cals := fetchAllCalendars()
	return c.JSON(http.StatusOK, cals)
}

func AdminCalendarDetail(c echo.Context) error {
	id := c.Param("id")
	cal := calendarWithEvents(id)
	return c.JSON(http.StatusOK, cal)
}

func AdminCalendarDetailCreate(c echo.Context) error {
	var cal model.CalendarDetail
	return c.JSON(http.StatusOK, cal)
}

func AdminCalendarDetailUpdate(c echo.Context) error {
	id := c.Param("id")
	cal := fetchCalendar(id)
	return c.JSON(http.StatusOK, cal)
}

func AdminCalendarDetailDelete(c echo.Context) error {
	id := c.Param("id")
	cal := fetchCalendar(id)
	return c.JSON(http.StatusOK, cal)
}

func AdminCalendarEvents(c echo.Context) error {
	events := fetchEvents()
	return c.JSON(http.StatusOK, events)
}

func AdminCalendarEvent(c echo.Context) error {
	id := c.Param("id")
	event := fetchEvent(id)
	return c.JSON(http.StatusOK, event)
}

type CalendarEvent struct {
	CalendarID  string `json:"calendar_id"`
	Summary     string `json:"summary"`
	Description string `json:"description"`
	Location    string `json:"location"`
	AllDay      bool   `json:"all_day"`
	StartAt     string `json:"start_at"`
	EndAt       string `json:"end_at"`
}

type CalendarEventRequest struct {
	CalendarEvent CalendarEvent `json:"calendar_event"`
}

func AdminCalendarEventCreate(c echo.Context) error {
	var request CalendarEventRequest
	if err := c.Bind(&request); err != nil {
		return err
	}
	e := model.CalendarEvent{
		AllDay:           request.CalendarEvent.AllDay,
		StartAt:          request.CalendarEvent.StartAt,
		EndAt:            request.CalendarEvent.EndAt,
		Location:         &request.CalendarEvent.Location,
		Summary:          request.CalendarEvent.Summary,
		Description:      request.CalendarEvent.Description,
		CalendarDetailId: request.CalendarEvent.CalendarID,
	}
	db := db.Conn()
	res := db.Create(&e)
	if res.Error != nil {
		fmt.Println(res.Error)
	}
	return c.JSON(http.StatusOK, e)
}

func AdminCalendarEventUpdate(c echo.Context) error {
	var request CalendarEventRequest
	if err := c.Bind(&request); err != nil {
		return err
	}

	id := c.Param("id")
	event := fetchEvent(id)
	update := model.CalendarEvent{
		AllDay:           request.CalendarEvent.AllDay,
		StartAt:          request.CalendarEvent.StartAt,
		EndAt:            request.CalendarEvent.EndAt,
		Location:         &request.CalendarEvent.Location,
		Summary:          request.CalendarEvent.Summary,
		Description:      request.CalendarEvent.Description,
		CalendarDetailId: request.CalendarEvent.CalendarID,
	}
	db := db.Conn()
	res := db.Model(&event).Updates(update)
	if res.Error != nil {
		fmt.Println(res.Error)
	}
	return c.JSON(http.StatusOK, event)
}

func AdminCalendarEventDelete(c echo.Context) error {
	id := c.Param("id")
	event := fetchEvent(id)
	fmt.Println(event)
	db := db.Conn()
	db.Delete(&event)
	return c.JSON(http.StatusOK, event)
}

package handler

import (
	"app/db"
	"app/model"
	"fmt"
)

func createCalendar(cal *model.CalendarDetail) *model.CalendarDetail {
	db := db.Conn()
	db.Create(&cal)
	return cal
}

func fetchRecentEvents() []model.CalendarEvent {
	db := db.Conn()
	var events []model.CalendarEvent
	db.Limit(10).Find(&events, "discarded_at IS NULL")
	return events
}

func fetchCalendar(id string) model.CalendarDetail {
	db := db.Conn()
	var cal model.CalendarDetail
	db.First(&cal, "id = ? AND discarded_at IS NULL", id)
	return cal
}

func fetchAllCalendars() []model.CalendarDetail {
	db := db.Conn()
	var cals []model.CalendarDetail
	db.Find(&cals, "discarded_at IS NULL")
	return cals
}

func calendarWithEvents(id string) model.CalendarDetail {
	db := db.Conn()
	var cal model.CalendarDetail
	err := db.Model(&model.CalendarDetail{}).Preload("Events").First(&cal, "id = ? AND discarded_at IS NULL", id).Error
	if err != nil {
		fmt.Println(err)
	}
	return cal
}

func fetchEvents() []model.CalendarEvent {
	db := db.Conn()
	var events []model.CalendarEvent
	db.Find(&events, "discarded_at IS NULL")
	return events
}

func fetchEvent(id string) model.CalendarEvent {
	db := db.Conn()
	var event model.CalendarEvent
	db.First(&event, "id = ? AND discarded_at IS NULL", id)
	return event
}

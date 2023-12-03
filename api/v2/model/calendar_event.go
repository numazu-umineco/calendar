package model

import (
	"app/util"
	"time"

	ics "github.com/arran4/golang-ical"
)

type CalendarEvent struct {
	Id               int32 `gorm:"primaryKey" json:"id"`
	AllDay           bool
	Description      string
	EndAt            string
	StartAt          string
	LastModifiedUser string
	Latitude         *float64
	Longitude        *float64
	Location         *string
	Summary          string
	CalendarDetailId string
	DiscardedAt      *string `gorm:"index"`
	CreatedAt        string
	UpdatedAt        string
}

func (e *CalendarEvent) startAt() time.Time {
	return util.TimeParse(e.StartAt)
}

func (e *CalendarEvent) endAt() time.Time {
	return util.TimeParse(e.EndAt)
}

func (e *CalendarEvent) createdAt() time.Time {
	return util.TimeParse(e.CreatedAt)
}

func (e *CalendarEvent) updatedAt() time.Time {
	return util.TimeParse(e.UpdatedAt)
}

func (e *CalendarEvent) Register(cal *ics.Calendar) {
	u := util.GenUUID()
	event := cal.AddEvent(u)
	event.SetCreatedTime(e.createdAt())
	event.SetStartAt(e.startAt())
	event.SetEndAt(e.endAt())
	event.SetSummary(e.Summary)
	event.SetLocation(*e.Location)
	event.SetDescription(e.Description)
	event.SetGeo(e.Latitude, e.Longitude)
	event.SetClass("PUBLIC")
}

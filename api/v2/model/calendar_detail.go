package model

import (
	ics "github.com/arran4/golang-ical"
	"github.com/samber/lo"
)

type CalendarDetail struct {
	Id          string `gorm:"primaryKey" json:"id"`
	Name        string
	DiscardedAt *string `gorm:"index"`
	CreatedAt   string
	UpdateAt    string
	Events      []CalendarEvent `gorm:"foreignKey:CalendarDetailId"`
}

func (c *CalendarDetail) ToIcal() string {
	cal := ics.NewCalendar()
	cal.SetMethod(ics.MethodRequest)
	lo.ForEach(c.Events, func(e CalendarEvent, i int) {
		e.Register(cal)
	})
	return cal.Serialize()
}

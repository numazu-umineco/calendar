package util

import (
	"time"

	"github.com/google/uuid"
)

func TimeParse(t string) time.Time {
	layout := "2006-01-02 15:04:05"
	tz, _ := time.LoadLocation("Asia/Tokyo")
	parsed, _ := time.ParseInLocation(layout, t, tz)
	return parsed
}

func GenUUID() string {
	return uuid.New().String()
}

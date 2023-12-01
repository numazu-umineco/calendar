package db

import (
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
)

func Conn() *gorm.DB {
	db, err := gorm.Open(sqlite.Open("database.sqlite3"))
	if err != nil {
		panic("failed to connect database")
	}
	return db
}

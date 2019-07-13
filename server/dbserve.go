package main

import (
	"database/sql"
	"fmt"
	"log"
	"net/http"

	_ "github.com/lib/pq"
)

// const (
// 	// localhost connection
// 	conninfo = "postgres://drbh2:@localhost:5432"
// )

const (
	host     = "localhost"
	port     = 5432
	user     = "drbh2"
	password = ""
	dbname   = "drbh2"
	addr     = "0.0.0.0:8888"
)

// open global DB connection instance
var psqlInfo = fmt.Sprintf("host=%s port=%d user=%s "+
	"password=%s dbname=%s sslmode=disable",
	host, port, user, password, dbname)

var db, err = sql.Open("postgres", psqlInfo)

func echo(w http.ResponseWriter, r *http.Request) {
	log.Println(r.URL)
}

func fetchLastRows(w http.ResponseWriter, r *http.Request) {
	results := db.QueryRow("SELECT IN_MS, REMAINING_BTC FROM POSTS ORDER BY IN_MS DESC LIMIT 1").Scan()
	log.Println(results)
}

type Row struct {
	In_ms         float64 `db:"in_ms"`
	Remaining_btc float64 `db:"remaining_btc"`
}

func main() {

	// check for errors
	if err != nil {
		panic(err)
	}

	// run query

	// IN_MS := 0
	// REMAINING_BTC := 0
	place := Row{}
	// var p []byte
	db.QueryRow("SELECT IN_MS, REMAINING_BTC FROM POSTS ORDER BY IN_MS DESC LIMIT 2").Scan(
		// &IN_MS, &REMAINING_BTC)
		&place.In_ms,
		&place.Remaining_btc)
	log.Println(&place)
	// log.Println(IN_MS, REMAINING_BTC)

	http.HandleFunc("/fetch", fetchLastRows)
	http.HandleFunc("/echo", echo)

	err := http.ListenAndServe(addr, nil)
	if err != nil {
		log.Fatal("ListenAndServe: ", err)
	}
}

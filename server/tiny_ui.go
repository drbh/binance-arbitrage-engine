package main

import "github.com/zserge/webview"

func main() {
	// Open wikipedia in a 800x600 resizable window
	webview.Open("BAE",
		"http://localhost:3000", 600, 800, true)
}

// Copyright 2015 The Gorilla WebSocket Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

// +build ignore

package main

import (
    "flag"
    // "html/template"
    "log"
    "net/http"

    "time"

    zmq "github.com/pebbe/zmq4"

    "github.com/gorilla/websocket"
)

var addr = flag.String("addr", "localhost:8888", "http service address")

var upgrader = websocket.Upgrader{} // use default options

// func doEvery(d time.Duration, f func(time.Time)) {
//     for x := range time.Tick(d) {
//         f(x)
//     }
// }

func echo(w http.ResponseWriter, r *http.Request) {

    //  Prepare our subscriber
    subscriber, err := zmq.NewSocket(zmq.SUB)

    if err != nil {
        log.Println("CON Error:", err)
    }

    defer subscriber.Close()

    // wait a second - incase the isse is connecting
    // to qickly after opening the socket (suggested on StackOverflow)
    time.Sleep(500 * time.Millisecond)

    // subscriber.Connect("tcp://*:5555")
    subscriber.Connect("tcp://localhost:5555")
    subscriber.SetSubscribe("")

    upgrader.CheckOrigin = func(r *http.Request) bool { return true }

    c, err := upgrader.Upgrade(w, r, nil)

    if err != nil {
        log.Print("upgrade:", err)
        return
    }

    defer c.Close()

    for {
        mt, message, err := c.ReadMessage()
        log.Println(mt)
        if err != nil {
            log.Println("read:", err)
            break
        }
        log.Printf("recv: %s", message)
        // err = c.WriteMessage(mt, message)

        for {
            // Read envelope with address
            address, err := subscriber.Recv(0)

            if err != nil {
                log.Println("MSG Error:", err)
                break
            }
            log.Println("send")
            c.WriteMessage(mt, []byte(address))
        }

        // if err != nil {
        //     log.Println("write:", err)
        //     break
        // }
    }
}

func main() {
    flag.Parse()
    log.SetFlags(0)
    http.HandleFunc("/echo", echo)
    log.Fatal(http.ListenAndServe(*addr, nil))
}

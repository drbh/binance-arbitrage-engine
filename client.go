//
//  Pubsub envelope subscriber.
//

package main

import (
	zmq "github.com/pebbe/zmq4"

	"fmt"
	"time"
)

func main() {
	//  Prepare our subscriber
	subscriber, err := zmq.NewSocket(zmq.SUB)

	if err != nil {
		fmt.Println("CON Error:", err)
	}

	defer subscriber.Close()

	// wait a second - incase the isse is connecting
	// to qickly after opening the socket (suggested on StackOverflow)
	time.Sleep(500 * time.Millisecond)

	// subscriber.Connect("tcp://*:5555")
	subscriber.Connect("tcp://localhost:5555")
	subscriber.SetSubscribe("")

	for {
		// Read envelope with address
		address, err := subscriber.Recv(0)

		if err != nil {
			fmt.Println("MSG Error:", err)

		}
		fmt.Println(address)
	}
}

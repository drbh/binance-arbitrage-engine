// subber.js
var zmq = require('zeromq')
  , sock = zmq.socket('sub');

sock.connect('tcp://localhost:5555');
sock.subscribe('');
console.log('Subscriber connected to port 5555');

sock.on('message', function(topic, message) {
	console.log(topic.toString('utf8'))
});
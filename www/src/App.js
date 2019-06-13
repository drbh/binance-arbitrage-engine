import React from 'react';
// import logo from './logo.svg';
import './App.css';
import { LineChart } from 'react-chartkick'
import 'chart.js'
import Websocket from 'react-websocket';

// JavaScript for drawing on canvas
// applying colors + three triangles



class App extends React.Component {
  constructor(){
    super()

    this.state = {
      variable_btcusdt: 0,
      variable_ethusdt: 0,
      variable_ethbtc: 0,
      variable_final: 0,
      historical: [],
      line: []
    }
    this.handleData = this.handleData.bind(this)
    this.draw = this.draw.bind(this)
  }

  handleData(data) {

    var pricing = JSON.parse(data)
    console.log(pricing)

    // this.refWebSocket.send("yo")
    // let vdat = JSON.parse(data)
    // console.log(vdat)
    var old_history = this.state.historical.slice(-99)
    var old_line = this.state.line
    var mydate = new Date()
    // old_history[mydate] = pricing.remaining_btc 
    old_history.push([mydate, pricing.remaining_btc-1])
    old_line = [[old_history[0][0], 0], [mydate, 0]]
    console.log(old_line)
   
    this.setState({
      variable_btcusdt: pricing.btcusdt,
      variable_ethusdt: pricing.ethusdt,
      variable_ethbtc: pricing.ethbtc,
      variable_final: pricing.remaining_btc,
      historical: old_history,
      line: old_line

    });

    this.draw()
  }

  draw() {
    // canvas with id="myCanvas"
    var canvas = document.getElementById('myCanvas');
    if (canvas.getContext) {
      var ctx = canvas.getContext('2d');
      
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      
      ctx.beginPath(); // note usage below 
      ctx.fillStyle = "#777";
      
      var x = 100
      var y = 100
      ctx.moveTo(x+0,  y+200); // start at top left corner of canvas
      ctx.lineTo(x+100,y+0); // go 200px to right (x), straight line from 0 to 0
      ctx.lineTo(x+200,y+200); // go to horizontal 100 (x) and vertical 200 (y)
      ctx.fill(); // connect and fil

      ctx.font = '24px sans-serif';
      
      ctx.fillText('BTC',x+0-60,  y+200);
      ctx.fillText('USD',x+100-25,y+0-15);
      ctx.fillText('ETH',x+200+15,y+200);
      
      ctx.font = '18px sans-serif';
      ctx.fillText('$ '+this.state.variable_btcusdt,60,  200);
      ctx.fillText('$ '+this.state.variable_ethusdt,270,  200);
      ctx.fillText('Ƀ '+this.state.variable_ethbtc,150,  330);
      
      ctx.font = '18px sans-serif';
      //ctx.fillStyle = "#000";
      ctx.fillStyle = "#F9A520";
      ctx.fillText(this.state.variable_final,158,  230);

    }
  }

  componentDidMount(){
    this.draw()
  }

  render(){
    return (
      <div className="App">
        <header className="App-header">


          <canvas id="myCanvas" width="400" height="400">
          </canvas>

          <LineChart 
          points={false}
          data={
          [
            {"name":"historical", "data": this.state.historical},
            {"name":"action", "data": this.state.line}
          ]
          } />

          <Websocket url={'ws://'+window.location.hostname+':8888/echo'}
          onOpen={() =>{
            this.refWebSocket.sendMessage("feed me data!")
          }}
          onMessage={this.handleData}
          ref={Websocket => {
              this.refWebSocket = Websocket;
            }}
          />

        </header>
      </div>
    );
  }
}

export default App;

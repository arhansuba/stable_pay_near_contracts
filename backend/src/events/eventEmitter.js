// eventEmitter.js
const EventEmitter = require('events');

class CustomEventEmitter extends EventEmitter {}

// Instantiate the EventEmitter and export it for use across the application
const customEventEmitter = new CustomEventEmitter();

module.exports = customEventEmitter;
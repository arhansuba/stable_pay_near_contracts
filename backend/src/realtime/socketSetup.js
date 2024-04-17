// socketSetup.js

const socketIo = require('socket.io');

/**
 * Initializes the WebSocket server.
 * @param {Object} server - The HTTP server instance.
 */
function setupWebSocket(server) {
  const io = socketIo(server);

  io.on('connection', (socket) => {
    console.log('A user connected:', socket.id);

    // Join a specific room related to user (e.g., based on userId)
    socket.on('joinRoom', (roomId) => {
      console.log(`Joining room: ${roomId}`);
      socket.join(roomId);
    });

    // Handling disconnect
    socket.on('disconnect', () => {
      console.log('User disconnected:', socket.id);
    });

    // You can add more event listeners here based on your application needs
  });
}

module.exports = { setupWebSocket };
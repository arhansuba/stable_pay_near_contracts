// WebSocketController.js
const WebSocket = require('ws');
const userService = require('./services/UserManagementService');

const clients = new Map();

function initWebSocketServer(server) {
    const wss = new WebSocket.Server({ server });

    wss.on('connection', (ws) => {
        console.log('Client connected');

        ws.on('message', async (message) => {
            console.log('Received message', message);
            const { userId, action } = JSON.parse(message);

            switch(action) {
                case 'REGISTER':
                    try {
                        const user = await userService.getUserDetails(userId);
                        clients.set(user.id, ws);
                        ws.send(JSON.stringify({ type: 'REGISTERED', userId: user.id }));
                    } catch (error) {
                        ws.send(JSON.stringify({ type: 'ERROR', message: 'Registration failed' }));
                    }
                    break;
                // Handle other actions...
                default:
                    console.log('Unknown action');
            }
        });

        ws.on('close', () => {
            console.log('Client disconnected');
            // Remove client from `clients` map
            clients.forEach((clientWs, clientId) => {
                if(clientWs === ws) {
                    clients.delete(clientId);
                }
            });
        });
    });

    // Example function to broadcast a new payment notification
    function broadcastNewPayment(paymentInfo) {
        clients.forEach(client => {
            if (client.readyState === WebSocket.OPEN) {
                client.send(JSON.stringify({ type: 'NEW_PAYMENT', data: paymentInfo }));
            }
        });
    }

    // Additional real-time functionalities as needed...

    return { wss, broadcastNewPayment };
}

module.exports = { initWebSocketServer };
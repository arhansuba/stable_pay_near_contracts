const { Server } = require("socket.io");

class NotificationService {
    constructor(server) {
        this.io = new Server(server, {
            cors: {
                origin: "*", // Adjust according to your CORS policy
            },
        });

        this.io.on("connection", (socket) => {
            console.log(`User connected: ${socket.id}`);

            socket.on("disconnect", () => {
                console.log(`User disconnected: ${socket.id}`);
            });

            // You can define additional socket event listeners here
        });
    }

    // Function to send a notification to a specific user
    sendNotificationToUser(userId, message) {
        // Assuming you have a way to map userIds to socketIds
        const socketId = this.getUserSocketId(userId);
        if (socketId) {
            this.io.to(socketId).emit("notification", message);
        }
    }

    // Example function to broadcast a notification to all connected users
    broadcastNotification(message) {
        this.io.emit("notification", message);
    }

    // Placeholder for a method that retrieves a socket ID based on a user ID
    // This part will need to be implemented based on your application's logic
    // for associating user IDs with socket connections.
    getUserSocketId(userId) {
        // Implementation depends on how you track connected users and their IDs
        // For example, you might keep a mapping of userIds to socketIds
        return null; // Replace with actual logic to return a socketId
    }
}

module.exports = NotificationService;
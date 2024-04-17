const express = require('express');
const userService = require('../services/userService');
const UserController = require ("../controllers/userController")
const router = express.Router();

// Route to create a new user
router.post('/users', async (req, res) => {
    try {
        const user = await userService.createUser(req.body);
        res.status(201).json(user);
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});
router.post('/register', UserController.registerUser);
router.post('/login', UserController.loginUser);
// Add routes for updating and fetching users similarly

module.exports = router;
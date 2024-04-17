// src/controllers/UserController.js
const User = require('../models/user');
const jwt = require('jsonwebtoken');
const { hashPassword, verifyPassword } = require('../utils/encryptionUtils');

exports.registerUser = async (req, res) => {
    const { email, password } = req.body;
    // Basic validation (consider using a library like Joi for more comprehensive validation)
    if (!email || !password) {
        return res.status(400).json({ message: "Email and password are required" });
    }

    try {
        // Check if user already exists
        const existingUser = await User.findOne({ email });
        if (existingUser) {
            return res.status(409).json({ message: "User already exists" });
        }

        const { salt, hash } = hashPassword(password);
        const newUser = new User({ email, passwordHash: hash, salt });
        await newUser.save();

        // Optionally, send back a token or user data
        res.status(201).json({ message: "User created successfully" });
    } catch (error) {
        console.error(error);
        res.status(500).json({ message: "Error creating the user" });
    }
};

exports.loginUser = async (req, res) => {
    const { email, password } = req.body;

    try {
        const user = await User.findOne({ email });
        if (!user || !verifyPassword(password, user.passwordHash, user.salt)) {
            return res.status(401).json({ message: "Invalid credentials" });
        }

        const token = jwt.sign({ userId: user._id }, process.env.JWT_SECRET, { expiresIn: '1h' });
        res.json({ message: "Login successful", token });
    } catch (error) {
        console.error(error);
        res.status(500).json({ message: "Error logging in" });
    }
};
// UserManagementService.js
const bcrypt = require('bcrypt');
const jwt = require('jsonwebtoken');
const { User } = require('../models/user'); // Assume User is a Mongoose model

const secretKey = process.env.JWT_SECRET_KEY;

async function register(userData) {
    const hashedPassword = await bcrypt.hash(userData.password, 10);
    const user = new User({ ...userData, password: hashedPassword });
    await user.save();
    return user;
}

async function login({ username, password }) {
    const user = await User.findOne({ username });
    if (!user || !(await bcrypt.compare(password, user.password))) {
        throw new Error('Invalid username or password');
    }
    return jwt.sign({ userId: user._id }, secretKey, { expiresIn: '1h' });
}

module.exports = { register, login };
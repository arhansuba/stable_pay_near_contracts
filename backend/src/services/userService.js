const User = require('../models/user');

exports.createUser = async (userData) => {
    try {
        const user = new User(userData);
        await user.save();
        return user;
    } catch (error) {
        console.error('Error creating the user:', error);
        throw error;
    }
};

exports.updateUser = async (accountId, updateData) => {
    try {
        const user = await User.findOneAndUpdate({ accountId }, updateData, { new: true });
        return user;
    } catch (error) {
        console.error('Error updating the user:', error);
        throw error;
    }
};

exports.getUser = async (accountId) => {
    try {
        const user = await User.findOne({ accountId });
        return user;
    } catch (error) {
        console.error('Error fetching the user:', error);
        throw error;
    }
};
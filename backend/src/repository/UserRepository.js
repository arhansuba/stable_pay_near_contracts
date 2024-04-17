// UserRepository.js
const User = require('../models/user'); // Assuming you have a Mongoose model for User

class UserRepository {
  async createUser(userData) {
    const user = new User(userData);
    await user.save();
    return user;
  }

  async findUserById(userId) {
    return User.findById(userId);
  }

  async updateUser(userId, updateData) {
    return User.findByIdAndUpdate(userId, updateData, { new: true });
  }

  async deleteUser(userId) {
    return User.findByIdAndDelete(userId);
  }

  // Add more methods as needed...
}

module.exports = new UserRepository();
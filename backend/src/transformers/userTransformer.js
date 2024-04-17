// userTransformer.js

/**
 * Transforms user data for client-side use.
 * @param {Object} user - The user object from the database.
 * @returns {Object} - The transformed user object.
 */
function transform(user) {
    return {
      id: user.id,
      username: user.username,
      email: user.email,
      createdAt: user.createdAt,
      // You can add more fields that are safe to send to the client.
      // Make sure not to expose sensitive information like passwords.
    };
  }
  
  module.exports = { transform };
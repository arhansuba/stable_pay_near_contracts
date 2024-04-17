// databaseConfig.js
const mongoose = require('mongoose');

const dbURI = process.env.MONGODB_URI || 'mongodb://localhost:27017/decentralizedVenmo';

const options = {
  useNewUrlParser: true,
  useUnifiedTopology: true,
  useCreateIndex: true,
  useFindAndModify: false,
};

mongoose.connect(dbURI, options)
  .then(() => console.log('Database connection established.'))
  .catch(err => console.error('Database connection error:', err));

module.exports = mongoose;
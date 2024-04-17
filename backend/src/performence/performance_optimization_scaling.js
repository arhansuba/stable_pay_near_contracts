// Import necessary modules
const express = require('express');
const cluster = require('cluster');
const totalCPUs = require('os').cpus().length;
const NodeCache = require('node-cache');
const { MongoClient } = require('mongodb');

// Initialize cache
const myCache = new NodeCache({ stdTTL: 100, checkperiod: 120 });

// Efficient Database Queries
// Optimizing database queries by selecting only necessary fields
async function fetchUserData(userId) {
  const client = await MongoClient.connect('mongodb://localhost:27017');
  const db = client.db('yourDatabase');
  
  // Efficient query: specifying fields to return
  const user = await db.collection('users').findOne({ _id: userId }, { projection: { password: 0 } });
  
  await client.close();
  return user;
}

// Caching Techniques
// Using in-memory cache to store frequently accessed data
async function getCachedUserData(userId) {
  const cachedUser = myCache.get(userId);
  if (cachedUser) return cachedUser;

  // If not in cache, fetch from the database and then cache it
  const user = await fetchUserData(userId);
  myCache.set(userId, user, 10000); // Cache for 10 seconds
  return user;
}

if (cluster.isMaster) {
  console.log(`Master ${process.pid} is running`);

  // Fork workers
  for (let i = 0; i < totalCPUs; i++) {
    cluster.fork();
  }

  cluster.on('exit', (worker, code, signal) => {
    console.log(`worker ${worker.process.pid} died`);
    console.log("Let's fork another worker!");
    cluster.fork();
  });
} else {
  // Workers can share any TCP connection
  // Here, it is an Express app
  const app = express();

  app.get('/data', async (req, res) => {
    const userId = req.query.userId;
    const cachedData = await getCachedUserData(userId);

    if (cachedData) {
      console.log('Serving from cache');
      return res.json(cachedData);
    } else {
      try {
        const client = await MongoClient.connect('mongodb://localhost:27017', { useNewUrlParser: true, useUnifiedTopology: true });
        const db = client.db('yourDB');
        const data = await db.collection('yourCollection').findOne({ userId });
        
        // Setting data to cache
        myCache.set(userId, data);
        
        console.log('Serving from database');
        res.json(data);
      } catch (error) {
        console.error(error);
        res.status(500).send('Internal Server Error');
      }
    }
  });

  const PORT = process.env.PORT || 3000;
  app.listen(PORT, () => console.log(`Worker ${process.pid} started, listening on port ${PORT}...`));
}

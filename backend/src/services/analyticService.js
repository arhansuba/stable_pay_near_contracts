const mongoose = require('mongoose');

const activitySchema = new mongoose.Schema({
  userId: String,
  action: String,
  timestamp: { type: Date, default: Date.now },
  metadata: mongoose.Schema.Types.Mixed,
});

const Activity = mongoose.model('Activity', activitySchema);

class AnalyticsService {
  async recordActivity(userId, action, metadata = {}) {
    const activity = new Activity({ userId, action, metadata });
    await activity.save();
  }

  async getActivitiesForUser(userId, { startDate, endDate }) {
    const query = { userId, timestamp: {} };
    if (startDate) query.timestamp.$gte = startDate;
    if (endDate) query.timestamp.$lt = endDate;
    return Activity.find(query);
  }

  // Additional methods for aggregating or querying specific analytics
}

module.exports = new AnalyticsService();
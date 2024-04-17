const cron = require('node-cron');
const { MongoClient } = require('mongodb');
const fs = require('fs-extra');
const shell = require('shelljs');

const dbUri = 'mongodb://localhost:27017';
const dbName = 'yourDatabaseName';
const backupPath = './backups';

// Create a backup of the database
async function backupDatabase() {
  // Ensure the backup directory exists
  await fs.ensureDir(backupPath);

  const timestamp = new Date().toISOString();
  const backupDir = `${backupPath}/backup-${timestamp}`;

  // Use MongoDB's native tools (mongodump) to backup the database
  shell.exec(`mongodump --uri="${dbUri}" --db=${dbName} --out=${backupDir}`, (code, stdout, stderr) => {
    if (code) {
      console.error(`Backup failed: ${stderr}`);
    } else {
      console.log(`Backup successful: ${stdout}`);
    }
  });
}

// Restore the database from a backup
async function restoreDatabase(backupDir) {
  // Use MongoDB's native tools (mongorestore) to restore the database
  shell.exec(`mongorestore --uri="${dbUri}" --db=${dbName} ${backupDir}/${dbName}`, (code, stdout, stderr) => {
    if (code) {
      console.error(`Restore failed: ${stderr}`);
    } else {
      console.log(`Restore successful: ${stdout}`);
    }
  });
}

// Schedule backups with node-cron
// This example schedules a backup every day at midnight
cron.schedule('0 0 * * *', () => {
  console.log('Running scheduled database backup...');
  backupDatabase();
});

module.exports = { backupDatabase, restoreDatabase };
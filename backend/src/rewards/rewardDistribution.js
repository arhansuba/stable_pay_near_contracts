const { connect, KeyPair, keyStores } = require('near-api-js');
const cron = require('node-cron');
const config = require('./config');

// Utilize UnencryptedFileSystemKeyStore for better key management
const keyStorePath = `./.near-credentials`;
const keyStore = new keyStores.UnencryptedFileSystemKeyStore(keyStorePath);

async function initNear() {
    const near = await connect({
        ...config,
        keyStore,
    });
    const account = await near.account(process.env.ACCOUNT_ID);
    return { account };
}

async function distributeRewards() {
    const { account } = await initNear();

    try {
        const gas = '100000000000000'; // 100 TeraGas, adjust based on contract need
        const attachedDeposit = '1'; // Attach 1 yoctoNEAR as a placeholder

        const result = await account.functionCall({
            contractId: config.contractId,
            methodName: 'distribute_rewards',
            args: {}, // Additional arguments
            gas,
            attachedDeposit,
        });

        console.log(`Rewards distributed successfully in tx ${result.transaction.hash}`);
    } catch (error) {
        console.error('Failed to distribute rewards:', error);
    }
}

// Flexible scheduling based on environment variables
const CRON_SCHEDULE = process.env.REWARDS_CRON_SCHEDULE || '0 0 * * *'; // Default: daily at midnight
cron.schedule(CRON_SCHEDULE, distributeRewards);
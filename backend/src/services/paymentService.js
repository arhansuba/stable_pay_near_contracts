const { connect, Contract, keyStores, WalletConnection } = require('near-api-js');
const config = require('../config'); // Assume this file contains your NEAR configuration

// Initialize NEAR connection
async function initNear() {
    const nearConfig = {
        networkId: config.networkId,
        keyStore: new keyStores.InMemoryKeyStore(),
        nodeUrl: config.nodeUrl,
        walletUrl: config.walletUrl,
        helperUrl: config.helperUrl,
        explorerUrl: config.explorerUrl,
    };

    try {
        const near = await connect(nearConfig);
        const wallet = new WalletConnection(near);
        return { near, wallet };
    } catch (error) {
        console.error("Error initializing NEAR connection:", error);
        throw error; // Rethrow the error to handle it outside this function
    }
}

// Function to send payment
exports.sendPayment = async (senderId, receiverId, amount) => {
    try {
        const { wallet } = await initNear();
        
        // Specifying the contract you're interacting with
        const contract = new Contract(wallet.account(), config.paymentContractName, {
            // View methods are read-only – they don't modify the state, but usually return some value
            viewMethods: [],
            // Change methods can modify the state, but you don't receive the returned value when called
            changeMethods: ['send_payment'],
        });

        // Calling a change method on the contract
        await contract.send_payment({ receiver_id: receiverId, amount: amount }, config.GAS, config.attachedDeposit);
        console.log("Payment sent successfully");
    } catch (error) {
        console.error("Error sending payment:", error);
        throw error; // Rethrow the error to handle it outside this function
    }
};

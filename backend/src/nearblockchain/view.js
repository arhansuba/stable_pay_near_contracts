async function callViewMethod() {
    const contract = await getContractInstance("devacctestnet.testnet"); // Use the accountId that deployed the contract or an authorized accountId
    try {
        const result = await contract.getViewData({ /* method arguments if any */ });
        console.log('View Method Result:', result);
    } catch (error) {
        console.error('Error calling view method:', error);
    }
}
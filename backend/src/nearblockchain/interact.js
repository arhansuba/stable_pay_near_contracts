const { Contract } = require('near-api-js');

async function fetchData() {
    const near = await initNear();
    const contract = new Contract(walletConnection.account(), 'devacctestnet.testnet', {
        viewMethods: ['getSomeData'], // List your contract's view methods here
        changeMethods: [],
    });

    const data = await contract.getSomeData(); // Replace with your contract's method
    console.log(data);
}
const contract = new Contract(walletConnection.account(), 'devacctestnet.testnet', {
    viewMethods: [],
    changeMethods: ['updateSomeData'], // List your contract's change methods here
});

async function updateData() {
    const response = await contract.updateSomeData({ arg1: 'value1' }, GAS, attachedDeposit);
    console.log(response);
}
async function getContractInstance(accountId) {
    const near = await initNear();
    const account = await near.account(accountId); // Account ID that will interact with the contract
    const contract = new Contract(account, "devacctestnet.testnet", {
        viewMethods: ["getViewData"], // Replace with your contract's view methods
        changeMethods: ["changeData"], // Replace with your contract's change methods
    });
    return contract;
}
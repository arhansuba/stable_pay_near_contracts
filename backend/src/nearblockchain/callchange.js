async function callChangeMethod() {
    const contract = await getContractInstance("devacctestnet.testnet");
    // Note: This assumes the backend account is authorized to make changes.
    // In real-world scenarios involving user accounts, transaction signing happens in the frontend.
    try {
        const result = await contract.changeData({ /* method arguments if any */ }, "300000000000000", /* attached deposit in yoctoNEAR, e.g., "1000000000000000000000000" for 1 NEAR */);
        console.log('Change Method Result:', result);
    } catch (error) {
        console.error('Error calling change method:', error);
    }
}
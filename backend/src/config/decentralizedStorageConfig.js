module.exports = {
    ipfs: {
        gatewayUrl: process.env.IPFS_GATEWAY_URL, // e.g., "https://ipfs.infura.io:5001/api/v0"
        publicGatewayUrl: process.env.IPFS_PUBLIC_GATEWAY_URL, // e.g., "https://ipfs.io/ipfs/"
    },
    filecoin: {
        networkUrl: process.env.FILECOIN_NETWORK_URL, // Example Filecoin network URL
        // Additional Filecoin-specific configurations if necessary
    },
    // Potentially include configurations for other decentralized storage networks here
};
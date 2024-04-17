// decentralized_storage_integration.js
const IPFS = require('ipfs-http-client');
const fs = require('fs');

// Connect to an IPFS node endpoint
const ipfs = IPFS.create({ url: 'https://ipfs.infura.io:5001' }); // You can use a local node or a public gateway

/**
 * Uploads a file to IPFS.
 * @param {string} filePath Path of the file to upload.
 * @returns {Promise<string>} The CID of the uploaded file.
 */
async function uploadFileToIPFS(filePath) {
    try {
        const file = fs.readFileSync(filePath);
        const { cid } = await ipfs.add({ path: filePath, content: file });
        console.log(`File uploaded successfully. CID: ${cid}`);
        return cid.toString();
    } catch (error) {
        console.error('Error uploading file to IPFS:', error);
        throw error;
    }
}

/**
 * Retrieves the content of a file from IPFS.
 * @param {string} cid The CID of the file to retrieve.
 * @returns {Promise<Buffer>} The content of the file.
 */
async function getFileFromIPFS(cid) {
    try {
        const chunks = [];
        for await (const chunk of ipfs.cat(cid)) {
            chunks.push(chunk);
        }
        console.log('File retrieved successfully from IPFS.');
        return Buffer.concat(chunks);
    } catch (error) {
        console.error('Error retrieving file from IPFS:', error);
        throw error;
    }
}

module.exports = { uploadFileToIPFS, getFileFromIPFS };
// SmartContractEventService.js
const { providers } = require('near-api-js');
const nearConfig = require('./config/nearConfig');

class SmartContractEventService {
  constructor() {
    this.provider = new providers.JsonRpcProvider({ url: nearConfig.nodeUrl });
  }

  // Listen for events from the blockchain
  async listenToEvents(contractId, fromBlock) {
    let lastBlock = fromBlock;
    setInterval(async () => {
      const currentBlock = await this.provider.block({ finality: 'final' });
      const blockHeight = currentBlock.header.height;
      if (lastBlock < blockHeight) {
        const changes = await this.provider.contractStateChanges({
          blockId: blockHeight,
          changesType: 'data_changes',
          accountId: contractId,
          // Optionally filter keys
        });
        this.processEvents(changes);
        lastBlock = blockHeight;
      }
    }, 10000); // Check every 10 seconds. Adjust as necessary.
  }

  processEvents(events) {
    events.forEach((event) => {
      // Process each event. This is where you might invoke other services,
      // send notifications, or trigger other actions in response to events.
      console.log('Event received:', event);
      
      // Example processing
      if (event.eventType === 'Transfer') {
        // handle transfer event
      }
    });
  }

  // Additional methods as needed for specific event types or processing logic...
}

module.exports = new SmartContractEventService();
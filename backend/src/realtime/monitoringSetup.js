const promBundle = require('express-prom-bundle');
const metricsMiddleware = promBundle({
  includeMethod: true,
  includePath: true,
  customLabels: { project_name: 'your_project_name' },
  promClient: {
    collectDefaultMetrics: {},
  },
});

module.exports = metricsMiddleware;
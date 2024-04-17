const xss = require('xss-clean');

const xssProtectionMiddleware = (req, res, next) => {
    req.body = req.body && xss(req.body);
    req.query = req.query && xss(req.query);
    req.params = req.params && xss(req.params);
    next();
};

module.exports = xssProtectionMiddleware;
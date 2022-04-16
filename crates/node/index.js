var promisify = require("util").promisify;
var _a = require("./index.node"), fromConfig = _a.fromConfig, log = _a.log, debug = _a.debug, info = _a.info, warning = _a.warning, error = _a.error, critical = _a.critical, shutdown = _a.shutdown;
var logAsync = promisify(log);
var debugAsync = promisify(debug);
var infoAsync = promisify(info);
var warningAsync = promisify(warning);
var errorAsync = promisify(error);
var criticalAsync = promisify(critical);
var Rollbar = /** @class */ (function () {
    function Rollbar(config) {
        this.instance = fromConfig(config);
    }
    Rollbar.prototype.log = function (level, message, extra) {
        return logAsync.call(this.instance, level, message, extra);
    };
    Rollbar.prototype.debug = function (message, extra) {
        return debugAsync.call(this.instance, 'debug', message, extra);
    };
    Rollbar.prototype.info = function (message, extra) {
        return infoAsync.call(this.instance, 'info', message, extra);
    };
    Rollbar.prototype.warning = function (message, extra) {
        return warningAsync.call(this.instance, 'warning', message, extra);
    };
    Rollbar.prototype.error = function (message, extra) {
        return errorAsync.call(this.instance, 'error', message, extra);
    };
    Rollbar.prototype.critical = function (message, extra) {
        return criticalAsync.call(this.instance, 'critical', message, extra);
    };
    Rollbar.prototype.shutdown = function () {
        return shutdown.call(this.instance);
    };
    return Rollbar;
}());
module.exports = Rollbar;

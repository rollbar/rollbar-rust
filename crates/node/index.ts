const { promisify } = require("util")

const {
    fromConfig,
    log,
    debug,
    info,
    warning,
    error,
    critical,
    shutdown,
} = require("./index.node")

const logAsync = promisify(log)
const debugAsync = promisify(debug)
const infoAsync = promisify(info)
const warningAsync = promisify(warning)
const errorAsync = promisify(error)
const criticalAsync = promisify(critical)

interface Config {
    accessToken: string
    endpoint?: string
}

type Level = 'debug' | 'info' | 'warning' | 'error' | 'critical'

interface ExtraData {
    [key: string]: any
}

class Rollbar {
    private instance: any

    constructor(config: Config) {
        this.instance = fromConfig(config)
    }

    log(level: Level, message: string, extra: ExtraData) {
        return logAsync.call(this.instance, level, message, extra)
    }

    debug(message: string, extra: ExtraData) {
        return debugAsync.call(this.instance, 'debug', message, extra)
    }

    info(message: string, extra: ExtraData) {
        return infoAsync.call(this.instance, 'info', message, extra)
    }

    warning(message: string, extra: ExtraData) {
        return warningAsync.call(this.instance, 'warning', message, extra)
    }

    error(message: string, extra: ExtraData) {
        return errorAsync.call(this.instance, 'error', message, extra)
    }

    critical(message: string, extra: ExtraData) {
        return criticalAsync.call(this.instance, 'critical', message, extra)
    }

    shutdown() {
        return shutdown.call(this.instance)
    }
}

module.exports = Rollbar

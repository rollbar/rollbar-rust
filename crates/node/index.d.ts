declare const promisify: any;
declare const fromConfig: any, log: any, debug: any, info: any, warning: any, error: any, critical: any, shutdown: any;
declare const logAsync: any;
declare const debugAsync: any;
declare const infoAsync: any;
declare const warningAsync: any;
declare const errorAsync: any;
declare const criticalAsync: any;
interface Config {
    accessToken: string;
    endpoint?: string;
}
declare type Level = 'debug' | 'info' | 'warning' | 'error' | 'critical';
interface ExtraData {
    [key: string]: any;
}
declare class Rollbar {
    private instance;
    constructor(config: Config);
    log(level: Level, message: string, extra: ExtraData): any;
    debug(message: string, extra: ExtraData): any;
    info(message: string, extra: ExtraData): any;
    warning(message: string, extra: ExtraData): any;
    error(message: string, extra: ExtraData): any;
    critical(message: string, extra: ExtraData): any;
    shutdown(): any;
}

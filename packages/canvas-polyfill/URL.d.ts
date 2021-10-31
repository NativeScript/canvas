declare export class URL {
    constructor(url: string, base?: string | URL);

    readonly native: any;


    hash: string;

    host: string;

    hostname: string;

    href: string;

    readonly origin: string;

    password: string;

    pathname: string;

    port: string;

    protocol: string;

    search: string;

    username: string;

    toJSON(): string;
    
    static createObjectURL(object: any, options = null): string;

    static revokeObjectURL(url: string);
}

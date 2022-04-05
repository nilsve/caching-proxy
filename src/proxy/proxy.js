function FindProxyForURL(url, host) {
    // Only pass localhost traffic through the proxy
    if (isPlainHostName(host)) {
        return 'HTTP localhost:8080';
    } else {
        return 'DIRECT';
    }
}
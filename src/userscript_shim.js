// Vencord and Equicord ship as userscripts that expect a userscript manager.
// LemonCord is the "manager" here, so it provides the two APIs they use.
var unsafeWindow = window;

// They only use GM_xmlhttpRequest as a fetch with responseType "blob".
// Unlike a real manager this can't bypass Discord's CSP, so requests to other sites may fail.
function GM_xmlhttpRequest(details) {
    fetch(details.url, { method: details.method, headers: details.headers, body: details.data })
        .then(async (res) => {
            const responseHeaders = [...res.headers].map(([k, v]) => `${k}: ${v}`).join("\r\n");
            details.onload?.({ status: res.status, responseHeaders, response: await res.blob() });
        })
        .catch(() => details.onerror?.());
}

// LemonCord's version of the SheltupdateNative bridge that sheltupdate's preload.js exposes, so
// sheltupdate's own "Client Mods" page (selector-ui.js) runs here unchanged.
// window.__lemoncord is set by LemonCord: { branches, host, available, supported }.
(() => {
    const state = window.__lemoncord;

    // Same shape preload.js builds, from the server's full list so the page matches Discord's.
    // A supported branch missing from it (list never fetched) falls back to its bare name.
    const server = Array.isArray(state.available) ? state.available : [];
    const stubs = state.supported
        .filter((id) => !server.some((b) => b.name === id))
        .map((id) => ({ name: id, displayName: id, description: "", type: "mod", incompatibilities: [], enabled: true }));

    // Branches LemonCord can't run stay listed like on Discord, but selector-ui.js greys them out.
    const trusted = Object.fromEntries(
        [...server, ...stubs].map((b) => {
            const lemoncordUnsupported = !state.supported.includes(b.name);
            const desc = lemoncordUnsupported ? `${b.description} (Not supported on this client as of yet.)` : b.description;
            return [b.name, { ...b, name: b.displayName, desc, lemoncordUnsupported }];
        })
    );
    state.branches = state.branches.filter((b) => b in trusted);

    // Plugins can call this API too, so changes are validated and confirmed like sheltupdate does.
    const confirmChange = (message) => {
        if (!window.confirm(message)) throw new Error("[lemoncord] User declined mods change");
    };

    window.SheltupdateNative = {
        getAvailableBranches: async () => trusted,
        getCurrentBranches: async () => [...state.branches],

        setBranches: async (branches) => {
            if (!Array.isArray(branches) || branches.some((b) => typeof b !== "string" || !Object.keys(trusted).includes(b)))
                throw new Error("[lemoncord] invalid branches passed to setBranches");

            confirmChange(`Confirm that you want to change your installed mods to: ${branches.map((b) => trusted[b].name).join(", ")}?`);
            window.ipc.postMessage("branches:" + branches.join("+"));
            state.branches = branches;
        },

        getCurrentHost: async () => state.host,

        setCurrentHost: async (host) => {
            let url;
            try {
                url = new URL(host);
            } catch {}
            if (!url || url.pathname !== "/" || host.endsWith("/") || (url.protocol !== "http:" && url.protocol !== "https:"))
                throw new Error("[lemoncord] invalid host passed to setCurrentHost");

            confirmChange(`Confirm that you want to change your sheltupdate instance to ${host}?`);
            window.ipc.postMessage("host:" + host);
            state.host = host;
        },

        uninstall: async () => {
            confirmChange("Confirm that you want to uninstall all client mods? Your settings will not be deleted.");
            window.ipc.postMessage("branches:");
            state.branches = [];
        },
    };
})();

(() => {
	const {
		plugins: { removePlugin },
		solid: { createSignal, createMemo, onCleanup, untrack },
		solidH: { html },
		ui: {
			Text,
			TextTags,
			TextWeights,
			LinkButton,
			Divider,
			Button,
			ButtonColors,
			ButtonSizes,
			TextBox,
			showToast,
			ToastColors,
			Header,
			HeaderTags,
			SwitchItem,
			Space,
			openModal,
			ModalRoot,
			ModalHeader,
			ModalBody,
			ModalFooter,
			ModalConfirmFooter,
			ModalSizes,
		},
	} = shelter;

	const {
		settings: { BadgeType, registerSection },
	} = shelter.plugin.scoped;

	const ClientModsIcon = html`
		<svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
			<path
				d="M13.833 1C14.3393 1 14.75 1.41073 14.75 1.91699V2.83301H19.333C20.3456 2.83301 21.167 3.65447 21.167 4.66699V9.25H22.083C22.5893 9.25 23 9.66074 23 10.167C22.9998 10.6731 22.5892 11.083 22.083 11.083H21.167V12.917H22.083C22.5892 12.917 22.9998 13.3269 23 13.833C23 14.3393 22.5893 14.75 22.083 14.75H21.167V19.333C21.167 20.3456 20.3456 21.167 19.333 21.167H14.75V22.083C14.75 22.5893 14.3393 23 13.833 23C13.3269 22.9998 12.917 22.5892 12.917 22.083V21.167H11.083V22.083C11.083 22.5892 10.6731 22.9998 10.167 23C9.66074 23 9.25 22.5893 9.25 22.083V21.167H4.66699C3.65447 21.167 2.83301 20.3456 2.83301 19.333V14.75H1.91699C1.41073 14.75 1 14.3393 1 13.833C1.00018 13.3269 1.41084 12.917 1.91699 12.917H2.83301V11.083H1.91699C1.41084 11.083 1.00018 10.6731 1 10.167C1 9.66074 1.41073 9.25 1.91699 9.25H2.83301V4.66699C2.83301 3.65447 3.65447 2.83301 4.66699 2.83301H9.25V1.91699C9.25 1.41073 9.66074 1 10.167 1C10.6731 1.00018 11.083 1.41084 11.083 1.91699V2.83301H12.917V1.91699C12.917 1.41084 13.3269 1.00018 13.833 1ZM5.125 18.875H18.875V5.125H5.125V18.875ZM14.75 7.41699C15.7625 7.41699 16.583 8.23748 16.583 9.25V14.75C16.583 15.7625 15.7625 16.583 14.75 16.583H9.25C8.23748 16.583 7.41699 15.7625 7.41699 14.75V9.25C7.41699 8.23748 8.23748 7.41699 9.25 7.41699H14.75ZM9.70801 14.292H14.292V9.70801H9.70801V14.292Z"
				fill="currentColor" />
		</svg>
	`;

	// deal with this plugin still existing after sheltupdate is gone!
	if (!window.SheltupdateNative) {
		const id = shelter.plugin.id;
		setTimeout(() => removePlugin(id), 50);
		return; // don't init or anything
	}

	const [branchMetaRaw, setBranchMetaRaw] = createSignal();
	const [branchMetaGrouped, setBranchMetaGrouped] = createSignal();
	const failedBranches = () => Object.values(branchMetaRaw() ?? {}).filter((branch) => branch.enabled === false);
	const [currentBranches, setCurrentBranches] = createSignal();

	const [currentHost, setCurrentHost] = createSignal();

	// just because i don't trust myself, keep a copy of the branch before uninstalling sheltupdate.
	const [uninstallCache, setUninstallCache] = createSignal();

	const [vencordOtherwiseLoaded, setVencordOtherwiseLoaded] = createSignal(false);
	const [bdOtherwiseLoaded, setBdOtherwiseLoaded] = createSignal(false);

	const showErrorToast = (title, error) => {
		console.error(`[sheltupdate] ${title}`, error);
		showToast({
			title,
			color: ToastColors.CRITICAL,
			content: "The change could not be applied.",
			duration: 5000,
		});
	};

	const updateCurrent = () =>
		Promise.all([
			SheltupdateNative.getCurrentBranches().then(setCurrentBranches),
			SheltupdateNative.getCurrentHost().then(setCurrentHost),
		]);
	updateCurrent().then(() => {
		if (window.Vencord && !currentBranches().includes("vencord") && !currentBranches().includes("equicord"))
			setVencordOtherwiseLoaded(true);

		if (window.BdApi && !currentBranches().includes("betterdiscord")) setBdOtherwiseLoaded(true);
	});

	let lastState;
	const updateClientModsSection = (newState) => {
		if (lastState === newState) return;

		 lastState = newState;
		 registerSection("section", "sheltupdate", "Client Mods", SettingsView, {
			icon: ClientModsIcon,
			badge: newState ? { type: BadgeType.WARNING } : undefined,
		 });
	};

	const updateAvailableBranches = () =>
		SheltupdateNative.getAvailableBranches().then((branches) => {
			// group by type, conveniently "mod" is before "tool" alphabetically
			const grouped = {};
			for (const branchName in branches) {
				const data = branches[branchName];
				if (!grouped[data.type]) grouped[data.type] = {};

				grouped[data.type][branchName] = data;
			}

			setBranchMetaGrouped(grouped);
			setBranchMetaRaw(branches);
			updateClientModsSection(Object.values(branches).some((branch) => branch.enabled === false));
		});

	const prettyModNames = (branches) => {
		const modNames = [...branches.filter((b) => b !== "shelter").map((b) => branchMetaRaw()[b].name)];

		// make sure that shelter always comes first
		if (branches.includes("shelter")) {
			modNames.unshift("shelter");
		}

		if (modNames.length === 1) return modNames[0];
		if (modNames.length === 2) return modNames.join(" and ");
		const lastMod = modNames.pop();
		return modNames.join(", ") + ", and " + lastMod;
	};

	// ok so this will display *above* the shelter heading which is not ideal but its okay i guess
	registerSection("divider");
	registerSection("header", "Sheltupdate");
	updateClientModsSection(false);
	updateAvailableBranches();

	function BranchEntry(props /*: { name, data, value, onChange } */) {
		// note: if shelter is disabled (i.e. you uninstalled sheltupdate), allow switching back on
		const disabled = () => {
			// LemonCord change: branches it can't load are greyed out (still switchable off if already on).
			if (props.data.lemoncordUnsupported && !props.value) {
				return "Not supported on this client as of yet.";
			}
			if (props.data.enabled === false && !props.value) {
				return "This branch is temporarily unavailable due to a server-side error.";
			}
			if (props.name === "shelter" && props.value) {
				return "You need shelter to have access to this menu. Try uninstalling sheltupdate.";
			}
			if ((props.name === "vencord" || props.name === "equicord") && vencordOtherwiseLoaded()) {
				return "Vencord or Equicord are currently loaded by some other mechanism.";
			}
			if (props.name === "betterdiscord" && bdOtherwiseLoaded()) {
				return "BetterDiscord is currently loaded by some other mechanism.";
			}
		};

		return html`
			<${SwitchItem}
				value=${() => props.value}
				onChange=${(e) => props.onChange?.(e)}
				note=${() => props.data.desc}
				disabled=${() => !!disabled()}
				tooltip=${() => disabled()}
			>
				${() => props.data.name}
			<//>
		`;
	}

	function SettingsView() {
		updateAvailableBranches();
		const updateTimer = setInterval(updateAvailableBranches, 10_000);
		onCleanup(() => clearInterval(updateTimer));

		// a Set<string> of branches
		const [pendingBranches, setPendingBranches] = createSignal(new Set(currentBranches()));

		// basically a set inequality test
		const hasUnsavedChanges = (e) => {
			const a = pendingBranches();
			const b = new Set(currentBranches());
			return a.size !== b.size || !a.isSubsetOf(b);
		};
		const isUninstalled = () => !currentBranches().length;

		// this is really silly
		return () => {
			if (isUninstalled()) {
				return html`<${UninstalledSplash} />`;
			}

			return html`
			  <${Header} tag=${HeaderTags.H1} style="margin-bottom: 1rem">Client Mod Settings<//>

				${() =>
					failedBranches().length
						? html`
							<div style=${{
						    "box-sizing": "border-box",
						    "padding": "16px",
						    "color": "var(--text-muted)",
						    "width": "100%",
						    "border-radius": "8px",
						    "background": "var(--background-feedback-warning)",
						    "border": "1px solid var(--border-feedback-warning)",
						    "margin-top": "12px",
							 "margin-bottom": "12px"
				       	}}>
								<${Header} tag=${HeaderTags.HeadingMD} margin=${false}>Some branches are currently unavailable due to server-side failure.<//>
								<div style=${{ "margin-top": "8px" }}/>
								${() => failedBranches().map((branch) => html`<${Text}>• ${branch.displayName}</${Text}><br/>`)}
						</div>
						`
						: null}

				<${Text}>
					Your installation of ${() => prettyModNames(currentBranches())} is being managed by
					<${Space} />
					<${LinkButton} href="https://github.com/uwu/sheltupdate">sheltupdate<//>.
					You can change the mods you are loading below.
				<//>

			  <${Divider} mt mb />

			  <!-- lol .map(), what is this, react? -->
			  ${() =>
					Object.values(branchMetaGrouped())
						.flatMap(Object.entries)
						.map(
							([branchName, branchData]) => html`
						<${BranchEntry}
							name=${() => branchName}
							data=${() => branchData}
							value=${() => pendingBranches().has(branchName)}
							onChange=${async (e) => {
								const pb = pendingBranches();
								if (e) {
									const foundIncompatibilities = branchData.incompatibilities.filter((i) => pb.has(i));
									if (foundIncompatibilities.length > 0) {
										const res = await openIncompatibilityModal(
											branchData.displayName,
											foundIncompatibilities,
										);
										if (res === "cancel") {
											return;
										}
									}
									pb.add(branchName);
								} else {
									pb.delete(branchName);
								}
								// reactivity ugh
								setPendingBranches(new Set(pb));
							}}
						/>
					`,
						)}

			  <div style=${{
					display: "flex",
					"flex-direction": "column",
					"align-items": "flex-end",
					"margin-top": "32px",
					gap: "16px",
				}}>
			  	  <${ChangeInstance} />
				  <${Button}
					  grow
					  color=${ButtonColors.RED}
					  onClick=${(e) => {
							setUninstallCache(currentBranches());
							SheltupdateNative.uninstall().then(updateCurrent, (err) => {
								updateCurrent();
								showErrorToast("Failed to change mods!", err);
							});
						}}
					  style=${{ "margin-left": "1rem" }}
				  >
					  Uninstall shelter
				  <//>
			  </div>

			  <${UnsavedChanges}
			  	hasUnsavedChanges=${hasUnsavedChanges}
				onReset=${(e) => {
					setPendingBranches(new Set(currentBranches()));
				}}
				onSave=${(e) => {
					SheltupdateNative.setBranches([...pendingBranches()]).then(
						() => {
							updateCurrent();
							showToast({
								title: "Please fully restart Discord!",
								color: ToastColors.WARNING,
								content: "Applying requires fully closing and reopening Discord.",
								duration: 5000,
							});
						},
						(err) => {
							updateCurrent();
							showErrorToast("Failed to change mods!", err);
						},
					);
				}}
				>
			  <//>
			`;
		};
	}

	function UninstalledSplash() {
		return html`
			<div style="display: flex; flex-direction: column; align-items: center; height: calc(100vh - 140px); text-align: center; justify-content: center; gap: .5rem;">
				<${Header} tag=${HeaderTags.H1} style="margin-bottom: 1rem">
					sheltupdate will be uninstalled at the next Discord app restart
				<//>
				<${Text}>
					Your plugins, themes, settings, etc. have not been deleted and will be remembered if
					you reinstall sheltupdate in the future, or switch to some other injection method.
				<//>
				<${Text}>
					Changed your mind? Until you restart Discord, you can retrieve your installation of
					<${Space} />
					${() => prettyModNames(uninstallCache())}
					<${Space} />
					exactly as it was before.
				<//>

				<${Button}
					grow
					color=${ButtonColors.GREEN}
					onClick=${(e) =>
						SheltupdateNative.setBranches(uninstallCache()).then(updateCurrent, (err) => {
							updateCurrent();
							showErrorToast("Failed to change mods!", err);
						})}
					style=${{ "margin-top": "2rem" }}
				>
					Revert uninstall
				<//>
			</div>
		`;
	}

	function UnsavedChanges(props) {
		return html`
			<div>
				<div style=${() => ({
					display: props.hasUnsavedChanges() ? "block" : "none",
					"max-width": "696px",
					width: "stretch",
					"min-width": "300px",
					bottom: "16px",
					"margin-right": "16px",
					position: "fixed",
					"z-index": "2",
				})}>
					<div style=${{
						"background-color": "var(--background-surface-highest, #2D2D2F)",
						border: "1px solid var(--border-subtle, rgba(151, 151, 155, 0.12))",
						"border-radius": "var(--radius-sm, 8px)",
						"box-shadow":
							"var(--legacy-elevation-high, 0 2px 10px 0 var(--opacity-black-20, rgba(0, 0, 0, 0.2)))",
						"padding-block": "10px",
						"padding-inline": "16px 10px",
					}}>
						<div style=${{
							display: "flex",
							"align-items": "center",
							"justify-content": "space-between",
							gap: "8px",
							position: "relative",
						}}>
							<div style=${{ overflow: "hidden" }}>
								<${Text}
									tag=${TextTags.textMD}
									weight=${TextWeights.medium}
									style=${{ "text-overflow": "ellipsis", "white-space": "nowrap", overflow: "hidden", display: "block" }}>
									Careful - you have unsaved changes!
								<//>
							</div>
							<div style=${{ display: "flex", "flex-grow": "0", gap: "0 10px", "justify-content": "end" }}>
								<${Button}
									grow
									color=${ButtonColors.SECONDARY}
									size=${ButtonSizes.SMALL}
									onClick=${(e) => props.onReset()}
								>
									Reset
								<//>
								<${Button}
									grow
									color=${ButtonColors.ACTIVE}
									size=${ButtonSizes.SMALL}
									onClick=${(e) => props.onSave()}
								>
									Save Changes
								<//>
							</div>
						</div>
					</div>
				</div>
			</div>
		`;
	}

	function ChangeInstance() {
		return html`
			<div style=${{
				display: "flex",
				"flex-direction": "row",
				"align-items": "center",
				gap: "8px",
			}}>
			<${Text} tag=${TextTags.textMD} style=${{ color: "var(--text-subtle)" }}>
				Configured sheltupdate instance:
				<${Space} />
				<${LinkButton} href=${currentHost}>
					${currentHost}
				<//>
				<//>
				<${Button}
					grow
					color=${ButtonColors.SECONDARY}
					onClick=${(e) =>
						openHostChangeModal().then((v) =>
							SheltupdateNative.setCurrentHost(v).then(updateCurrent, (err) => {
								updateCurrent();
								showErrorToast("Failed to change host!", err);
							}),
						)}
				>Change</Button>
			</div>
		`;
	}

	async function openIncompatibilityModal(branchName, incompatibleBranches) {
		return new Promise((resolve) => {
			openModal(({ close }) => {
				onCleanup(() => resolve("cancel"));
				const incompatibleBranchNames = prettyModNames(incompatibleBranches);
				return html`
					<${ModalRoot}>
						<${ModalHeader} close=${(e) => close()}>
							Things might break!
						<//>
						<${ModalBody}>
							<b>${branchName}</b> may not work properly alongside <b>${incompatibleBranchNames}</b>.<br /><br />
							Do you want to proceed anyways?
						<//>
						<${ModalFooter}>
							<div style="display: flex; justify-content: flex-end; gap: .5rem;">
								<${Button}
									color=${ButtonColors.SECONDARY}
									size=${ButtonSizes.MEDIUM}
									grow
									onClick=${(e) => {
										resolve("cancel");
										close();
									}}
								>
									Cancel
								<//>
								<${Button}
									color=${ButtonColors.RED}
									size=${ButtonSizes.MEDIUM}
									grow
									onClick=${(e) => {
										resolve("proceed");
										close();
									}}
								>
									Proceed anyways
								<//>
							</div>
						<//>
					<//>
				`;
			});
		});
	}

	async function openHostChangeModal() {
		return new Promise((res, rej) =>
			openModal(({ close }) => {
				onCleanup(rej);

				const [newHost, setNewHost] = createSignal(untrack(currentHost));

				const validationIssue = createMemo(() => {
					// must be a URL
					let url;
					try {
						url = new URL(newHost());
					} catch {
						return "Not a valid URL";
					}

					// must not have a trailing path, else branch setting logic would break
					if (url.pathname !== "/") return "Hosts must not have a path";

					// don't have a trailing /
					if (newHost().endsWith("/")) return "Hosts must not have a trailing /";

					// openasar does not support http
					if (url.protocol !== "https:") {
						if (url.protocol === "http:") {
							if (window.openasar) return "OpenAsar does not work with insecure hosts";
						} else return "Hosts must be http: or https:";
					}
				});

				return html`
				<${ModalRoot} size=${ModalSizes.MEDIUM}>
					<${ModalHeader} close=${close}>Change sheltupdate instance<//>
					<${ModalBody}>
						<p><${Text}>
							You can host your own instance of sheltupdate, instead of using uwu.network's official instance,
							and switch to using it here.
						<//></p>
						<p><${Text}>
							We suggest using the official instance as it is always up-to-date,
							runs unmodified official sheltupdate code, and has high reliability.
							We run a
							<${Space} />
							<${LinkButton} onClick=${(e) => setNewHost("https://inject.shelter.uwu.network")}>stable instance<//>
							<${Space} />
							and a
							<${Space} />
							<${LinkButton} onClick=${(e) => setNewHost("https://staging.shelter.uwu.network")}>staging instance<//>.
						<//></p>
						<p><${Text}>
							If someone has told you to change this, be sure you trust them, as the server that you specify here
							can deliver code to you that will be run when you open Discord, with full access to your computer.
						<//></p>
						<${Divider} mt mb />

						<${TextBox}
							value=${newHost}
							onInput=${(v) => setNewHost(v)}
							style=${() => (validationIssue() ? { border: "1px solid var(--input-border-critical)" } : {})}
						/>
						<${Text} tag=${TextTags.textSM} style=${{ color: "var(--text-critical)" }}>
							${validationIssue}
						<//>
					<//>
					<${ModalConfirmFooter} type="danger" confirmText="Proceed" close=${() => close} disabled=${validationIssue} onConfirm=${() => res(newHost())} />
				<//>
			`;
			}),
		);
	}
})();

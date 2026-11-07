const orig = JSON.parse;
JSON.parse = function () {
	const res = orig.apply(this, arguments);
	["adPlacements", "adSlots", "playerAds"].forEach((key) => {
		if (key in res) {
			res[key] = [];
		}
	});
	return res;
};

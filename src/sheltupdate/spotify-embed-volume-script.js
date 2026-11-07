const speakerSvg = `
    <svg width="22" height="22" viewBox="0 0 24 24" fill="none">
        <path d="M15 20.3682V3.63181C15 2.88512 14.2168 2.40696 13.5615 2.76502C11.555 3.86146 7.95096 5.94303 6.06753 7.71429C4.57041 9.12223 2.89573 7.91678 2.34431 9.85714C1.88542 11.4719 1.88513 12.5281 2.34407 14.1429C2.89555 16.0832 4.57046 14.8778 6.06753 16.2857C7.95094 18.057 11.555 20.1386 13.5615 21.235C14.2168 21.5931 15 21.1149 15 20.3682Z" stroke="white" stroke-width="2.5"/>
        <path class="loud" d="M23 11.9989C23 5.98963 18 5 18 5V6.50195C18 6.50195 21.5 7.50802 21.5 11.9989C21.5 16.4898 18 17.5098 18 17.5098V19.008C18 19.008 23 18.0099 23 11.9989Z" fill="white" stroke="white"/>
        <path class="quiet" d="M18 14V10C18 10 19 10.0055 19 12.0014C19 14 18 14 18 14Z" fill="white" stroke="white"/>
    </svg>
`;
const volumeKey = "playbackVolume";

if (!localStorage.getItem(volumeKey)) {
	localStorage.setItem(volumeKey, 0.5);
}

let audio;
let volumeSlider;

const origPlay = Audio.prototype.play;
Audio.prototype.play = function () {
	audio = this;
	updateVolume(localStorage.getItem(volumeKey));
	return origPlay.apply(this, arguments);
};

function updateVolume(volume) {
	localStorage.setItem(volumeKey, volume);

	if (audio) audio.volume = volume;
	if (volumeSlider) volumeSlider.value = volume * 100;

	const loudCircle = document.querySelector("#inserted_volume_slider .loud");
	const quietCircle = document.querySelector("#inserted_volume_slider .quiet");
	if (!loudCircle || !quietCircle) return;
	loudCircle.style.display = volume > 0.5 ? "block" : "none";
	quietCircle.style.display = volume > 0 ? "block" : "none";
}

const controlsWrapperQuery = "[class^=PlayerControlsShort_playerControlsWrapper]";

const insertVolumeSlider = () => {
	const controlsWrapper = document.querySelector(controlsWrapperQuery);
	if (!controlsWrapper) return;
	if (controlsWrapper.querySelector("#inserted_volume_slider")) return;

	controlsWrapper.insertAdjacentHTML(
		"afterbegin",
		`
        <div id="inserted_volume_slider">            
            <input type="range" style="visibility: hidden;">
            <button>
                ${speakerSvg}
            </button>
        </div>
        `,
	);

	volumeSlider = controlsWrapper.querySelector("#inserted_volume_slider > input");
	volumeSlider.addEventListener("input", () => updateVolume(volumeSlider.value / 100));

	const volumeButton = controlsWrapper.querySelector("#inserted_volume_slider > button");
	volumeButton.addEventListener("click", () => updateVolume(0));
	volumeButton.addEventListener("mouseenter", () => (volumeSlider.style.visibility = "visible"));
	controlsWrapper.addEventListener("mouseleave", () => (volumeSlider.style.visibility = "hidden"));

	updateVolume(localStorage.getItem(volumeKey));
};

insertVolumeSlider();
const observer = new MutationObserver(insertVolumeSlider);
observer.observe(document, { subtree: true, attributes: true, attributeFilter: ["class"] });

const style = document.createElement("style");
style.innerText = `
#inserted_volume_slider > input[type="range"] {
    -webkit-appearance: none;
    background-color: rgba(255, 255, 255, .2);
    width: 80px;
    height: 12px;
    border-radius: 6px;
    overflow: hidden;
    cursor: pointer;

    &::-webkit-slider-thumb {
        -webkit-appearance: none;
        width: 0;
        border: 0;
        box-shadow: -80px 0 0 80px white;
    }
}    

#inserted_volume_slider > button {
    display: flex;
    cursor: pointer;
    &:hover {
        transform: scale(1.04);
    }
}

#inserted_volume_slider {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-bottom: 1px;
}`;

document.documentElement.append(style);

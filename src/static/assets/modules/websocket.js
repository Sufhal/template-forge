import {buildIframe, getIframe, replaceIframe} from "./renderer.js";
import {updateTemplatesSelect} from "./controls.js";
import {restoreSelect} from "./persistence.js";
import {templatesSelect} from "./ui.js";

const ws = new WebSocket('ws://localhost:8866/ws');

ws.addEventListener('message', e => {
    const templates = JSON.parse(e.data);
    for (const name in templates) {
        const template = templates[name];
        let iframe = getIframe(name);
        if (!iframe) {
            iframe = buildIframe(name);
        }
        replaceIframe(template, iframe);
    }
    updateTemplatesSelect(Object.keys(templates));
    restoreSelect(templatesSelect);
})
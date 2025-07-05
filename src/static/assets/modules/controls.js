import {renderer, templatesSelect, viewportSelect} from "./ui.js";
import {persistSelect, restoreSelect} from "./persistence.js";
import {getIframe} from "./renderer.js";

export function initControls() {
    viewportSelect.addEventListener('change', (e) => {
        const option = viewportSelect.options[viewportSelect.selectedIndex].value;
        switch (option) {
            case 'Mobile': {
                renderer.style.width = '390px';
                break;
            }
            case 'Tablet': {
                renderer.style.width = '768px';
                break;
            }
            case 'Desktop': {
                renderer.style.width = '100%';
                break;
            }
        }
        persistSelect(viewportSelect);
    });
    restoreSelect(viewportSelect);

    templatesSelect.addEventListener('change', (e) => {
        const template = templatesSelect.options[templatesSelect.selectedIndex].value;
        Array.from(templatesSelect.options)
            .map(v => getIframe(v.value))
            .forEach(v => v.style.display = 'none');
        getIframe(template).style.display = 'block';
        persistSelect(templatesSelect);
    });
}

export function updateTemplatesSelect(templates) {
    for (const name of templates) {
        const option = Array.from(templatesSelect.options).find(v => v.name === name);
        if (!option) {
            templatesSelect.append(createOption(name));
        }
    }
}

function createOption(name) {
    const option = document.createElement("option");
    option.value = name;
    option.text = name;
    return option;
}
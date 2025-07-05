import {renderer} from "./ui.js";

export function buildIframe(name) {
    const iframe = document.createElement('iframe');
    iframe.setAttribute("data-name", name);
    renderer.appendChild(iframe);
    return iframe;
}

export function getIframe(name) {
    return renderer.querySelector(`[data-name="${name}"]`);
}

export function replaceIframe(html, iframe) {
    const blob = new Blob([html], {type: 'text/html'});
    iframe.src = URL.createObjectURL(blob);
}
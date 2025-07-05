/**
 * @param {HTMLSelectElement} select
 */
export function persistSelect(select) {
    localStorage.setItem(getKey(select), String(select.options[select.selectedIndex].value));
}

/**
 * @param {HTMLSelectElement} select
 */
export function restoreSelect(select) {
    const desiredValue = localStorage.getItem(getKey(select));
    if (desiredValue) {
        const optionIndex = Array.from(select.options).findIndex(option => option.value === desiredValue);
        if (optionIndex !== -1) {
            select.selectedIndex = optionIndex;
        }
    }
    select.dispatchEvent(new Event('change', {bubbles: true}));
}

/**
 * @param {HTMLSelectElement} select
 */
function getKey(select) {
    return `persistence-${select.id}`;
}
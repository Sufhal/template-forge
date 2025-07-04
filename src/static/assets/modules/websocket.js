const ws = new WebSocket('ws://localhost:8866/ws');
const messages = document.getElementById('messages');
const messageInput = document.getElementById('messageInput');
const status = document.getElementById('status');

ws.addEventListener('message', e => {

})

ws.onopen = function (event) {
    status.textContent = 'Connected';
    status.style.color = 'green';
};

ws.onmessage = function (event) {
    const messageDiv = document.createElement('div');
    messageDiv.textContent = event.data;
    messages.appendChild(messageDiv);
    messages.scrollTop = messages.scrollHeight;
};

ws.onclose = function (event) {
    status.textContent = 'Disconnected';
    status.style.color = 'red';
};

ws.onerror = function (error) {
    status.textContent = 'Error';
    status.style.color = 'red';
};

function sendMessage() {
    const message = messageInput.value;
    if (message.trim() !== '') {
        ws.send(message);
        messageInput.value = '';
    }
}

messageInput.addEventListener('keypress', function (e) {
    if (e.key === 'Enter') {
        sendMessage();
    }
});
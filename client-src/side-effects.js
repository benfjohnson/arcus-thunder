// Side effects

// the websocket connection, which needs to be passed to certain side effects
let ws;

const move = (ws, player_id, direction) => ws.send(JSON.stringify({ player_id, direction }));

const listenForArrowKeys = (ws) => {
    const id = document.cookie
        .split('; ')
        .find(row => row.startsWith('atid'))
        .split('=')[1];

    window.addEventListener('keydown', (e) => {
        switch (e.keyCode) {
            case 37:
                move(ws, id, 'Left');
                break;
            case 38:
                move(ws, id, 'Up');
                break;
            case 39:
                move(ws, id, 'Right');
                break;
            case 40:
                move(ws, id, 'Down');
                break;
        }
    });
};

export const initiateWebsocket = dispatch => {
    ws = new WebSocket('ws://localhost:3000/connect');
    ws.onopen = () => console.log('opened a socket!');
    ws.onmessage = msg => dispatch({ type: 'GET_GAME', worldMap: JSON.parse(msg.data).world_map });
}

export const initDispatch = (model, update, view, render) => (action) => {
    model = update(model, action);
    render(view(model, sideEffects), document.querySelector('#app'));
};

export const authenticate = dispatch => (
    fetch('http://localhost:3000/auth', { credentials: 'include' })
);

export const sideEffects = { ws: () => ws, authenticate, initiateWebsocket, listenForArrowKeys };

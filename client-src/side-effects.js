// Side effects

import { paintCurrPositions } from './canvas.js';

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

const initiateWebsocket = async (ctx, render) => {
    return new Promise(resolve => {
        ws = new WebSocket('ws://localhost:3000/connect');
        ws.onopen = () => console.log('opened a socket!') || resolve();
        // TODO: Can view rendering be functional/declarative, a la react?
        ws.onmessage = msg => {
            const { world_map: worldMap, state: gameState } = JSON.parse(msg.data);
            render({ worldMap, gameState });
            paintCurrPositions(ctx, worldMap, gameState);
        };
    });
}

const authenticate = async () => (
    await fetch('http://localhost:3000/auth', { credentials: 'include' })
);

export const sideEffects = { ws: () => ws, authenticate, initiateWebsocket, listenForArrowKeys };

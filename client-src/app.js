import { html, render } from 'https://unpkg.com/lit-html@1.2.1/lit-html.js';
import { initDispatch, sideEffects } from './side-effects.js';

const worldMap = [];

const player = 'White';

let model = { worldMap, player };

const update = (model, action) => {
    switch(action.type) {
        case 'GET_GAME':
            model.worldMap = action.worldMap;
            return model;
        case 'GET_PLAYER_COLOR':
            model.player = action.color;
            return model;
        default:
            return model;
    }
};

const view = (model, sideEffects) => html`
    <div>
        ${model.worldMap.map(row => {
            return html`
                ${row.map(pos => html`<span style=${pos && (`color:#${Number(pos.color).toString(16)}`)}>${(pos && `#${Number(pos.color).toString(16)}`) || '====='}</span>`)}
                <br/>
            `;
        })}
    </div>
`;

const dispatch = initDispatch(model, update, view, render);

render(view(model, sideEffects), document.querySelector('#app'));

const ws = new WebSocket('ws://localhost:3000/connect');

ws.onopen = () => console.log('opened a socket!');
ws.onmessage = (msg) => console.log('ben2', msg.data) || dispatch({ type: 'GET_GAME', worldMap: JSON.parse(msg.data).world_map });

// side effects to trigger on startup
sideEffects.authenticate(dispatch)
    .then(() => {
        sideEffects.listenForArrowKeys(ws);
    });

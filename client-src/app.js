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
                ${row.map(pos => html`<span>${pos || '====='}</span>`)}
                <br/>
            `;
        })}
    </div>
`;

const dispatch = initDispatch(model, update, view, render);

render(view(model, sideEffects), document.querySelector('#app'));

// side effects to trigger on startup
sideEffects.getGame(dispatch)();
sideEffects.selectPlayerFromQuerystring(dispatch)(window);
sideEffects.listenForArrowKeys(dispatch)(model.player);

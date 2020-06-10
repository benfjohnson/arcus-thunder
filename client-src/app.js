import { html, render } from 'https://unpkg.com/lit-html@1.2.1/lit-html.js';
import { initDispatch, sideEffects } from './side-effects.js';

const worldMap = [
    [null, null, null, null, null, null, null, null],
    [null, null, null, 'white', null, null, null, null],
    [null, null, null, null, null, null, null, null],
    [null, null, null, null, null, null, null, null],
    [null, null, null, null, null, null, null, null],
    [null, null, null, null, null, null, null, null],
    [null, null, null, null, null, null, null, null],
];

const player = 'white';

let model = { worldMap, player };

const update = (model, action) => {
    switch(action.type) {
        case 'GET_GAME':
            return Object.assign({}, model, { worldMap: action.worldMap });
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

sideEffects.listenForArrowKeys(dispatch)();

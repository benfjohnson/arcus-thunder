import { html, render } from 'https://unpkg.com/lit-html@1.2.1/lit-html.js';
import { styleMap } from 'https://unpkg.com/lit-html@1.2.1/directives/style-map.js';
import { sideEffects } from './side-effects.js';
import { initializeCanvas } from './canvas.js';

const view = state => html`
    <div>
        <canvas id='game-map' width='800' height='800'></canvas>
        <div id='score-card'>
            <h1>Score:</h1>
            ${state.worldMap.map(row => {
                return row.filter(Boolean).map(pos => html`<p style=${styleMap({color: `#${Number(pos.color).toString('16')}`})}><strong>Player: ${pos.score}</strong></p>`);
            })}
        </div>
    </div>
`;

const main = async () => {
    render(view({ worldMap: []}), document.querySelector('#app'));
    const ctx = document.querySelector('#game-map').getContext('2d');

    await initializeCanvas(ctx);

    // side effects to trigger on startup
    await sideEffects.authenticate();
    await sideEffects.initiateWebsocket(ctx, (state) => {
        render(view(state), document.querySelector('#app'));
    });
    sideEffects.listenForArrowKeys(sideEffects.ws());
};

main();

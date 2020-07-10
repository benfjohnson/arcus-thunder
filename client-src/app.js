import { html, render } from 'https://unpkg.com/lit-html@1.2.1/lit-html.js';
import { sideEffects } from './side-effects.js';
import { initializeCanvas } from './canvas.js';

const view = () => html`
    <div>
        <canvas id='game-map' width='800' height='800'></canvas>
    </div>
`;

const main = async () => {
    render(view(), document.querySelector('#app'));
    const ctx = document.querySelector('#game-map').getContext('2d');

    await initializeCanvas(ctx);

    // side effects to trigger on startup
    await sideEffects.authenticate();
    await sideEffects.initiateWebsocket(ctx);
    sideEffects.listenForArrowKeys(sideEffects.ws());
};

main();

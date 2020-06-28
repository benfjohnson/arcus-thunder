// Side effects

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

export const initDispatch = (model, update, view, render) => (action) => {
    model = update(model, action);
    render(view(model, sideEffects), document.querySelector('#app'));
};

export const authenticate = dispatch => (
    fetch('http://localhost:3000/auth', { credentials: 'include' })
        .then(res => res.json())
        .then(data => console.log('ben', data))
);

export const sideEffects = { authenticate, listenForArrowKeys };

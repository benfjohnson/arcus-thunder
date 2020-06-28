// Side effects

const getGame = dispatch => () => {
    fetch('http://localhost:3000/game')
        .then(res => res.json())
        .then(data => dispatch({ type: 'GET_GAME', worldMap: data.world_map }));
};

const move = (ws, player, direction) => ws.send(JSON.stringify({ player, direction }));

const listenForArrowKeys = (ws, color) => {
    window.addEventListener('keydown', (e) => {
        switch (e.keyCode) {
            case 37:
                move(ws, color, 'Left');
                break;
            case 38:
                move(ws, color, 'Up');
                break;
            case 39:
                move(ws, color, 'Right');
                break;
            case 40:
                move(ws, color, 'Down');
                break;
        }
    });
};

const selectPlayerFromQuerystring = dispatch => window => {
    const searchParams = new URLSearchParams(window.location.search);
    const maybePlayerColor = searchParams.get('player');
    // For now just safeguard against undefined behavior by returning black as the default color:
    dispatch({ type: 'GET_PLAYER_COLOR', color: maybePlayerColor || 'White' });
};

export const initDispatch = (model, update, view, render) => (action) => {
    model = update(model, action);
    render(view(model, sideEffects), document.querySelector('#app'));
};

export const authenticate = dispatch => {
    fetch('http://localhost:3000/auth', { credentials: 'include' })
        .then(res => res.json())
        .then(data => console.log('ben', data));
}

export const sideEffects = { authenticate, getGame, listenForArrowKeys, selectPlayerFromQuerystring };

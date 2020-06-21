// Side effects

const getGame = dispatch => () => {
    fetch('http://localhost:3000/game')
        .then(res => res.json())
        .then(data => dispatch({ type: 'GET_GAME', worldMap: data.world_map }));
};

const move = dispatch => (player, direction) => {
    const putParams = {
        headers: {
            'Content-Type': 'application/json',
        },
        method: 'PUT',
        body: JSON.stringify({ player, direction })
    };
    fetch('http://localhost:3000/game', putParams)
        .then(getGame(dispatch)());
};

const listenForArrowKeys = dispatch => color => {
    window.addEventListener('keydown', (e) => {
        switch (e.keyCode) {
            case 37:
                move(dispatch)(color, 'Left');
                break;
            case 38:
                move(dispatch)(color, 'Up');
                break;
            case 39:
                move(dispatch)(color, 'Right');
                break;
            case 40:
                move(dispatch)(color, 'Down');
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

export const sideEffects = { getGame, move, listenForArrowKeys, selectPlayerFromQuerystring };

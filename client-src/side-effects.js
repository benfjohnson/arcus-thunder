// Side effects

const getGame = dispatch => () => {
    fetch('http://localhost:3000/game')
        .then(res => res.json())
        .then(data => dispatch({ type: 'GET_GAME', worldMap: data.world_map }));
};

const move = dispatch => (player, direction) => {
    const putParams = { method: 'PUT', body: JSON.stringify({ player, direction }) };
    fetch('http://localhost:3000/game', putParams)
        .then(getGame(dispatch)());
};

export const initDispatch = (model, update, view, render) => (action) => {
    model = update(model, action);
    render(view(model, sideEffects), document.querySelector('#app'));
};

export const sideEffects = { getGame, move };

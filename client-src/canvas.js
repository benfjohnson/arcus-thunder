const WIDTH_PX = 800;
const HEIGHT_PX = 800;
// The number of "spaces" to evenly distribute across our canvas
const GAME_SPACES = 8;
const SPACE_PX = WIDTH_PX / GAME_SPACES;

let grassTileCache = null;
const getImg = async () => {
    if (grassTileCache) return Promise.resolve(grassTileCache);

    return new Promise(res => {
        const gt = new Image();
        gt.src = './assets/grass-tile.png';
        gt.onload = () => {
            grassTileCache = gt;
            res(gt);
        };
    });
};

export const initializeCanvas = async ctx => {
    const grassTile = await getImg();
    const backgroundPattern = ctx.createPattern(grassTile, 'repeat');
    ctx.fillStyle = backgroundPattern;
    ctx.fillRect(0, 0, WIDTH_PX, HEIGHT_PX);
};

export const paintCurrPositions = async (ctx, board) => {
    ctx.clearRect(0, 0, WIDTH_PX, HEIGHT_PX);
    await initializeCanvas(ctx);
        board.forEach((row, y) => {
            row.forEach((pos, x) => {
                if (!pos) return;
                ctx.fillStyle = `#${Number(pos.color).toString(16)}`;
                ctx.beginPath();
                ctx.arc(x * SPACE_PX + (SPACE_PX / 2), y * SPACE_PX + (SPACE_PX / 2), SPACE_PX / 2, 0, 2 * Math.PI);
                ctx.stroke();
                ctx.fill();
            });
        });
};

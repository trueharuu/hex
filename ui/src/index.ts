import { ansiBlock } from "./ansi_block";
import { mirage } from "./ayu";
import { readdir, stat } from 'fs/promises';
import { gradient } from "./gradient";
const app = Bun.serve({
    port: 1025, routes: {
        '/': () => {
            return new Response('hi');
        },

        '/test': () => {
            let htm = `<!DOCTYPE html><html><head></head><body>${ansiBlock('\x1b[1;31mHello\x1b[0m \x1b[34mWorld\x1b[0m!')}</body></html>`;
            return new Response(htm, {
                headers: {
                    'Content-Type': 'text/html'
                }
            });
        },

        '/sets': async () => {
            const dir = await readdir('./output/sets');

            // render table where each column is player X and each row is player O, and each cell is a link to the set page for that pair of players
            // the value of each cell is the sum of wins, where if X wins +1, if O wins -1, and if draw 0


            const players = dir.map(entry => entry.split('-')[0]).filter((v, i, a) => a.indexOf(v) === i).sort();
            let htm = `<!DOCTYPE html><html><head></head><body style='margin: 30px; background-color: ${mirage.ui.bg.hex()}; color: ${mirage.terminal.white.hex()}; font-size: 14pt; font-family: "Source Serif 4";'><table style='border-collapse: collapse;'><tr><th style='border-right: 1px solid ${mirage.terminal.white.hex()};border-bottom: 1px solid ${mirage.terminal.white.hex()};'>X \\ O</th>`;

            const w = '2em';
            const p = '10em';
            const A = `max-width: ${w}; max-height: ${p}; width: ${w}; height: ${p};`;
            const B = `max-width: ${p}; max-height: ${w}; width: ${p}; height: ${w};`;
            const C = `max-width: ${w}; max-height: ${w}; width: ${w}; height: ${w};`;
            for (const player of players) {
                htm += `<th style='transform: rotate(-90deg); border-bottom: 1px solid ${mirage.terminal.white.hex()}; ${A} color: ${mirage.terminal.red.hex()};'>${player}</th>`;
            }
            htm += '</tr>';

            for (const o of players) {
                htm += `<tr><th style='${B} border-right: 1px solid ${mirage.terminal.white.hex()}; padding-right: 10px; color: ${mirage.terminal.blue.hex()};'>${o}</th>`;
                for (const x of players) {
                    const file = `./output/sets/${x}-${o}`;
                    try {
                        await stat(file);
                        const content = await readdir(file);
                        const n = content.length;
                        let score = 0;
                        for (const round of content) {
                            const f = Bun.file(`${file}/${round}`);
                            const txt = await f.text();
                            const result = txt.split('\n').find(line => line.startsWith('# '))?.split('# ')[1] || 'D';
                            if (result === 'X') {
                                score += 1;
                            } else if (result === 'O') {
                                score -= 1;
                            }
                        }
                        const color = gradient((score + n) / (2 * n), [mirage.terminal.blue.hex(), mirage.terminal.white.hex(), mirage.terminal.red.hex()]);
                        htm += `<td style='${C} text-align: center; background-color: ${color}22; '><a style='color: ${mirage.terminal.white.hex()};text-decoration: none;' href='/sets/${x}/${o}'>${score}</a></td>`;
                    } catch (e) {
                        htm += `<td style='${C} text-align: center; color: ${mirage.terminal.white.hex()};'>-</td>`;
                    }
                }
                htm += '</tr>';
            }

            htm += '</table></body></html>';

            return new Response(htm, {
                headers: {
                    'Content-Type': 'text/html'
                }
            });
        },

        '/sets/:x/:o': async (req) => {
            const { x, o } = req.params;
            const dir = await readdir(`./output/sets/${x}-${o}`);
            const rounds = dir.map(entry => entry.replace('.txt', '')).sort((a, b) => +a - +b);
            let htm = `<!DOCTYPE html><html><head></head><body style='margin: 30px; background-color: ${mirage.ui.bg.hex()}; color: ${mirage.terminal.white.hex()}; font-size: 14pt; font-family: "Source Serif 4";'><span style='color: ${mirage.terminal.red.hex()}; font-weight: bold;'>${x}</span> vs. <span style='color: ${mirage.terminal.blue.hex()}; font-weight: bold;'>${o}</span><br>`;

            for (const round of rounds) {
                const file = `./output/sets/${x}-${o}/${round}.txt`;
                const f = Bun.file(file);
                const content = await f.text();
                let result = content.split('\n').find(line => line.startsWith('# '))?.split('# ')[1] || 'D';
                let rt = '';
                if (result === 'D') {
                    rt = 'Draw';
                } else {
                    rt = result === 'X' ? 'X wins' : 'O wins';
                }
                console.log(file, result);
                const color = result === 'X' ? mirage.terminal.red.hex() : result === 'O' ? mirage.terminal.blue.hex() : mirage.terminal.white.hex();

                htm += `<div style='background-color: ${color}22; display: inline-flex; width: 100px; height: 100px; border-radius: 5px; margin: 5px; text-align: center; align-items: center; justify-content: center;'><a style='color: ${mirage.terminal.white.hex()}; text-decoration: none;' href='/sets/${x}/${o}/${round}'>${+round + 1}</a></div>`;

            }

            htm += '</body></html>';
            return new Response(htm, {
                headers: {
                    'Content-Type': 'text/html'
                }
            });
        },

        '/sets/:x/:o/:round': async (req) => {
            const { x, o, round } = req.params;
            const render = await Bun.$`./target/release/ascii_renderer ./output/sets/${x}-${o}/${round}.txt`;
            const txt = render.text();
            const htm = `<!DOCTYPE html><html><head></head><body style='margin: 30px; background-color: ${mirage.ui.bg.hex()}; color: ${mirage.terminal.white.hex()}; font-size: 14pt; font-family: "Source Serif 4";'><span style='color: ${mirage.terminal.red.hex()}; font-weight: bold;'>${x}</span> vs. <span style='color: ${mirage.terminal.blue.hex()}; font-weight: bold;'>${o}</span> (round ${+round + 1}) ${ansiBlock(txt)}</body></html>`;
            return new Response(htm, {
                headers: {
                    'Content-Type': 'text/html'
                }
            });
        }
    }
});
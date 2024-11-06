const fs = require('node:fs');

/*
let proms = [];
for (let i = 0; i<(37/25); i++) {
	const prom = fetch(`https://api.deezer.com/user/6144247603/playlists?index=${i*25}`)
		.then(a=>a.json())
		.then(({data})=>{
			return data.map(({id, title})=>({id, title}));
		})
	proms.push(prom);
}
const playlists = (await Promise.all(proms)).flat();
console.log(playlists)
//fs.writeFile("./playlists.json", JSON.stringify(titles));
*/

fs.readFile("./all.json", (e, ok)=>{
	const info = JSON.parse(ok);
	console.log(info);
});


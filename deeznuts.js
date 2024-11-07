const fs = require('node:fs');

let proms = [];
for (let i = 0; i<(611/25); i++) {
	const prom = fetch(`https://api.deezer.com/user/6144247603/tracks?index=${i*25}`)
		.then(a=>a.json())
		.then(({data})=>{
			return {
				music: data.map(dt=>({
					deezer_id: dt.id,
					name: dt.title,
					album: dt.album.title,
					duration: dt.duration,
					artist_id: dt.artist.id,
				})),
				artist: data.map(dt=>({
					deezer_id: dt.artist.id,
					name: dt.artist.name,
				}))
			};
		})
	proms.push(prom);
}
const tracks = [];
const artists = [];
const stuff = (await Promise.all(proms)).flat();
stuff.forEach(({music, artist})=>{
	tracks.push(...music)
	artists.push(...artist)
})
fs.writeFile("./liked.json", JSON.stringify({tracks, artists}, null, "  "), ()=>{});


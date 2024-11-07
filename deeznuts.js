const fs = require('node:fs');

function getAllLikedTracks() {
	return fetch("https://api.deezer.com/user/6144247603/tracks?limit=99999")
		.then(a=>a.json())
		.then(({data})=>(
			{
				tracks: data.map(dt=>({
					deezer_id: dt.id,
					name: dt.title,
					album: dt.album.title,
					duration: dt.duration,
					artist_id: dt.artist.id,
				})),
				artists: data.map(dt=>({
					deezer_id: dt.artist.id,
					name: dt.artist.name,
				}))
			}
		))
}

function getAllPlaylists() {
	return fetch("https://api.deezer.com/user/6144247603/playlists?limit=99999")
		.then(a=>a.json())
		.then(({data})=>({
			playlists: data.map(dt=>({
				deezer_id: dt.id,
				name: dt.title,
			}))
		}))
}

const x = await getAllPlaylists()
//console.log(x);
fs.writeFile("./playlists.json", JSON.stringify(x, null, "  "), ()=>{});

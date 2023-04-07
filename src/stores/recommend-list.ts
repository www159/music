import { atom } from "jotai";

export const atomPlaylists = atom<Playlist[]>([]);

export const atomPlaylistsSome = atom((get) => {
  const allPlaylists = get(atomPlaylists);
  return allPlaylists.slice(0, 6);
});

export const atomPlaylistId = atom<number>(0);
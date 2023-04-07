import { PlaylistInfo } from "@/components/playlist/playlist-info";
import { SongList } from "@/components/playlist/songlist";
import { getPlaylistDetail } from "@/services/invoke/rquest";
import { atomPlaylistId } from "@/stores/recommend-list";
import { Box, Paper, Stack, Table, TableBody, TableCell, TableContainer, TableHead, TableRow, Typography } from "@mui/material";
import { useAtom } from "jotai";
import { useEffect, useState } from "react";

function ms2mmss(time: number) {
  time = time / 1000;
  const minute = Math.floor(time / 60);
  const second = Math.floor(time % 60);
  return `${minute}:${second < 10 ? `0${second}` : second}`;
}

function textLimit(text: string | undefined) {
  if(!text) return "";
  let ret = text.trim();
  if(ret.length < 200) return ret;
  ret = ret.substring(0, 200);
  ret = ret + "...";
  return ret;
}

export const PlaylistPage = () => {

  // SECTION state
  const [playlistDetail, setPlaylistDetail] = useState<PlaylistDetail>();
  // ~SECTION

  // SECTION store
  const [plailistId] = useAtom(atomPlaylistId);
  // ~SECTION

  // SECTION initialize effect
  useEffect(() => {
    getPlaylistDetail({
      id: plailistId,
    })
      .then((playlistDetail) => {
        setPlaylistDetail(playlistDetail);
      });
  }, []);
  // ~SECTION
  return (
    <Stack
      sx={{
        marginRight: "1ch",
        height: "calc(100% - 50px)",
      }}
    >
      <Paper
        sx={{
          marginTop: "1ch"
        }}
      >
        <Stack
          direction={"row"}
          sx={{
            marginTop: "2ch",
            marginBottom: "2ch",
          }}
        >
          <Box
            component="img"
            src={playlistDetail?.coverImgUrl}
            alt="playlist cover img"
            sx={{
              width: "140px",
              height: "140px",
              marginLeft: "2ch"
            }}
          />
          <Stack
            sx={{
              marginLeft: "4ch",
            }}
          >
            <Typography
              fontSize="large"
            >
              {playlistDetail?.name}
            </Typography>
            <Typography
              fontSize={"small"}
              overflow={"hidden"}
            >
              {textLimit(playlistDetail?.description)}
            </Typography>
          </Stack>
        </Stack>
      </Paper>
      <TableContainer
        component={Paper}
        sx={{
          marginTop: "2ch"
        }}
      >
        <Table>
          <TableHead>
            <TableRow>
              <TableCell>name</TableCell>
              <TableCell>duration</TableCell>
              <TableCell>singer</TableCell>
              <TableCell>album</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {playlistDetail?.songs.map(song => (
              <TableRow key={song.id}>
                <TableCell>{song.name}</TableCell>
                <TableCell>{ms2mmss(song.duration)}</TableCell>
                <TableCell>{song.singers.join(",")}</TableCell>
                <TableCell>{song.album}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>
    </Stack>
  );
};
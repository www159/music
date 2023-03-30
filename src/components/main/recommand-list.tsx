import { listPlaylist } from "@/services/invoke/rquest";
import { atomPlaylists, atomPlaylistsSome } from "@/stores/recommend-list";
import { Avatar, Box, Grid, Paper, Stack, Typography } from "@mui/material";
import { useAtom } from "jotai";
import { useEffect } from "react";
import useSWR from "swr";
import { RecommandListToolbar } from "./recommand-list-toolbar";

// SECTION props
interface RecommandListProps {
  label: string
}
// ~SECTION
export const RecommandList = (props: RecommandListProps) => {
  const { label } = props;

  // SECTION store
  const [, setPlaylists] = useAtom(atomPlaylists);
  const [playlists, ] = useAtom(atomPlaylistsSome);
  // ~SECTION
  
  useEffect(() => {
    listPlaylist({})
      .then((playlists) => {
        setPlaylists(playlists);
      });
  }, []);
  return (
    <Stack
      sx={{
        // height: "1000px",
      }}
    >
      <RecommandListToolbar label={label} />
      <Paper 
        elevation={3}
        sx={{
          marginRight: "5px",
        }}
      >
        <Grid
          container
          columns={{ xs: 4, md: 12 }}
          sx={{
            marginTop: "6px",
          }}
        >
          {playlists.map(playlist => (
            <Grid 
              item
              xs={1}
              md={2} 
              sx={{
                display: "flex",
                alignItems: "center",
                flexDirection: "column",
              }}
              key={playlist.id}
            >
              <Avatar src={playlist.cover_img_url} />
              <Typography 
                sx={{
                  lineClamp: "2",
                  wordBreak: "break-all",
                  textOverflow: "ellipsis",
                  overflow: "hidden",
                  fontSize: 10,
                  width: "15ch",
                  marginTop: "1ch",
                }}
              >
                {playlist.name}
              </Typography>
            </Grid>
          ))}
        </Grid>
      </Paper>
    </Stack>
  );
};